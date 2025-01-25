use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    process,
};

mod licenses;

const VALID_LICENSES: &str = "\
Licenses and their valid identifiers. All identifiers are case-insensitive.

BSD Zero Clause:\n  BSD0, 0BSD, BSD-0\n
Academic Free License v3.0:\n  AFL-3.0, AFL-3, AFL3, AFL\n
GNU Affero General Public License v3.0:\n  AGPL-3.0, AGPL-3, AGPL3, AGPL\n
Apache License 2.0:\n  Apache-2.0, Apache2, Apache\n
Artistic License 2.0:\n  Artistic-2.0, Artistic2, Artistic\n
Blue Oak Model License 1.0.0:\n  BlueOak-1.0.0, BlueOak-1, BlueOak1, BlueOak\n
BSD-2-Clause Plus Patent License:\n  BSD-2-Clause-Patent, BSD-2-Patent, BSD2-Patent\n
BSD 2-Clause \"Simplified\" License:\n  BSD-2-Clause, BSD-2, BSD2\n
BSD 3-Clause Clear License:\n  BSD-3-Clause-Clear, BSD-3-Clear, BSD3-Clear\n
BSD 3-Clause \"New\" or \"Revised\" License:\n  BSD-3-Clause, BSD-3, BSD3\n
BSD 4-Clause \"Original\" or \"Old\" License:\n  BSD-4-Clause, BSD-4, BSD4\n
Boost Software License 1.0:\n  BSL-1.0, BSL1, BSL\n
Creative Commons Attribution 4.0 International:\n  CC-BY-4.0, CC-BY-4, CCBY4, CCBY\n
Creative Commons Attribution Share Alike 4.0 International:\n  CC-BY-SA-4.0, CC-BY-SA-4, CCBYSA4, CCBYSA\n
Creative Commons Zero v1.0 Universal:\n  CC0-1.0, CC0-1, CC0\n
CeCILL Free Software License Agreement v2.1:\n  CECILL-2.1, CECILL\n
CERN Open Hardware Licence Version 2 - Permissive:\n  CERN-OHL-P-2.0, CERN-OHL-P\n
CERN Open Hardware Licence Version 2 - Strongly Reciprocal:\n  CERN-OHL-S-2.0, CERN-OHL-S\n
CERN Open Hardware Licence Version 2 - Weakly Reciprocal:\n  CERN-OHL-W-2.0, CERN-OHL-W\n
Educational Community License v2.0:\n  ECL-2.0, ECL-2, ECL2, ECL\n
Eclipse Public License 1.0:\n  EPL-1.0, EPL-1, EPL1\n
Eclipse Public License 2.0:\n  EPL-2.0, EPL-2, EPL2, EPL\n
European Union Public License 1.1:\n  EUPL-1.1, EUPL1.1\n
European Union Public License 1.2:\n  EUPL-1.2, EUPL1.2, EUPL\n
GNU Free Documentation License v1.3:\n  GFDL-1.3, GFDL1.3, GFDL\n
GNU General Public License v2.0:\n  GPL-2.0, GPL-2, GPL2, GPLv2\n
GNU General Public License v3.0:\n  GPL-3.0, GPL-3, GPL3, GPLv3, GPL\n
ISC License:\n  ISC\n
GNU Lesser General Public License v2.1:\n  LGPL-2.1, LGPL-2, LGPL2\n
GNU Lesser General Public License v3.0:\n  LGPL-3.0, LGPL-3, LGPL3, LGPL\n
LaTeX Project Public License v1.3c:\n  LPPL-1.3c, LPPL-1.3, LPPL1.3, LPPL\n
MIT No Attribution:\n  MIT-0, MIT0\n
MIT License:\n  MIT\n
Mozilla Public License 2.0:\n  MPL-2.0, MPL-2, MPL2, MPL\n
Microsoft Public License:\n  MS-PL, MSPL\n
Microsoft Reciprocal License:\n  MS-RL, MSRL\n
Mulan Permissive Software License, Version 2:\n  MulanPSL-2.0, MulanPSL-2, MulanPSL2, MulanPSL, Mulan\n
University of Illinois/NCSA Open Source License:\n  NCSA, UIUC\n
Open Data Commons Open Database License v1.0:\n  ODbL-1.0, ODbL-1, ODbL1, ODbL\n
SIL Open Font License 1.1:\n  OFL-1.1, OFL1.1, OFL\n
Open Software License 3.0:\n  OSL-3.0, OSL-3, OSL3, OSL\n
PostgreSQL License:\n  PostgreSQL, PSQL\n
The Unlicense:\n  Unlicense\n
Universal Permissive License v1.0:\n  UPL-1.0, UPL-1, UPL1, UPL\n
Vim License:\n  Vim\n
Do What The Fuck You Want To Public License:\n  WTFPL\n
zlib License:\n  Zlib";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const TRY_HELP: &str = "Try 'license-picker --help' for more information";
const TRY_LIST: &str = "Try 'license-picker --list' for all license options";
const HELP: &str = "\
USAGE: license-picker [OPTION]... LICENSE...
Choose one or more LICENSEs for your project.

ARGUMENTS:
  LICENSE...
    License(s) to generate in the current directory. The default file name
    for a single license choice is `LICENSE`. Multiple licenses can be
    chosen, and each will be named to indicate what license it is. For example,
    choosing the MIT and Apache 2.0 licenses will create the files `LICENSE-MIT`
    and `LICENSE-APACHE`.

FLAGS:
  -h, --help          Display this help message and exit
  -V, --version       Display version information and exit
  -p, --print         Prints the license to standard output
  -l, --list          List license options and exit
  -c, --check         Check if the given license specifier(s) is valid

OPTIONS:
  -e, --extension EXT Sets the extension for the output file (default: None)
  -n, --name  NAME    The full name for the license(s) (default: None)
  -y, --year  YEAR    The year for the license(s) (default: None)
  -m, --email EMAIL   The email for the license(s) (default: None)
  -j, --project PROJ  The project for the license(s) (default: None)
  -u, --url URL       The project url for the license(s) (default: None)";

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
enum License {
    BSD0,
    AFL,
    AGPL,
    Apache,
    Artistic,
    BlueOak,
    BSD2Patent,
    BSD2,
    BSD3Clear,
    BSD3,
    BSD4,
    BSL,
    CCBY,
    CCBYSA,
    CC0,
    Cecill,
    CernOHLP,
    CernOHLS,
    CernOHLW,
    ECL,
    EPL1,
    EPL2,
    EUPL11,
    EUPL12,
    GFDL,
    GPL2,
    GPL3,
    ISC,
    LGPL2,
    LGPL3,
    LPPL,
    MIT0,
    MIT,
    MPL,
    MSPL,
    MSRL,
    MulanPSL,
    NCSA,
    ODbL,
    OFL,
    OSL,
    PostgreSQL,
    Unlicense,
    UPL,
    Vim,
    WTFPL,
    Zlib,
}

impl License {
    fn specifier(&self) -> &str {
        match self {
            License::BSD0 => "BSD0",
            License::AFL => "AFL",
            License::AGPL => "AGPL",
            License::Apache => "APACHE",
            License::Artistic => "ARTISTIC",
            License::BlueOak => "BLUEOAK",
            License::BSD2Patent => "BSD2-PATENT",
            License::BSD2 => "BSD2",
            License::BSD3Clear => "BSD3-CLEAR",
            License::BSD3 => "BSD3",
            License::BSD4 => "BSD4",
            License::BSL => "BSL",
            License::CCBY => "CC-BY",
            License::CCBYSA => "CC-BY-SA",
            License::CC0 => "CC0",
            License::Cecill => "CECILL",
            License::CernOHLP => "CERN-OHL-P",
            License::CernOHLS => "CERN-OHL-S",
            License::CernOHLW => "CERN-OHL-W",
            License::ECL => "ECL",
            License::EPL1 => "EPL-1.0",
            License::EPL2 => "EPL-2.0",
            License::EUPL11 => "EUPL-1.1",
            License::EUPL12 => "EUPL-1.2",
            License::GFDL => "GFDL",
            License::GPL2 => "GPLv2",
            License::GPL3 => "GPLv3",
            License::ISC => "ISC",
            License::LGPL2 => "LGPLv2.1",
            License::LGPL3 => "LGPLv3",
            License::LPPL => "LPPL-1.3c",
            License::MIT0 => "MIT-0",
            License::MIT => "MIT",
            License::MPL => "MPL",
            License::MSPL => "MS-PL",
            License::MSRL => "MS-RL",
            License::MulanPSL => "MULANPSL",
            License::NCSA => "NCSA",
            License::ODbL => "ODBL",
            License::OFL => "OFL",
            License::OSL => "OSL",
            License::PostgreSQL => "POSTGRESQL",
            License::Unlicense => "UNLICENSE",
            License::UPL => "UPL",
            License::Vim => "VIM",
            License::WTFPL => "WTFPL",
            License::Zlib => "ZLIB",
        }
    }
}

impl TryFrom<&str> for License {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use License::*;
        match value {
            "bsd0" | "0bsd" | "bsd-0" => Ok(BSD0),
            "afl-3.0" | "afl-3" | "afl3" | "afl" => Ok(AFL),
            "agpl-3.0" | "agpl-3" | "agpl3" | "agpl" => Ok(AGPL),
            "apache-2.0" | "apache2" | "apache" => Ok(Apache),
            "artistic-2.0" | "artistic2" | "artistic" => Ok(Artistic),
            "blueoak-1.0.0" | "blueoak-1" | "blueoak1" | "blueoak" => Ok(BlueOak),
            "bsd-2-clause-patent" | "bsd-2-patent" | "bsd2-patent" => Ok(BSD2Patent),
            "bsd-2-clause" | "bsd-2" | "bsd2" => Ok(BSD2),
            "bsd-3-clause-clear" | "bsd-3-clear" | "bsd3-clear" => Ok(BSD3Clear),
            "bsd-3-clause" | "bsd-3" | "bsd3" => Ok(BSD3),
            "bsd-4-clause" | "bsd-4" | "bsd4" => Ok(BSD4),
            "bsl-1.0" | "bsl1" | "bsl" => Ok(BSL),
            "cc-by-4.0" | "cc-by-4" | "ccby4" | "ccby" => Ok(CCBY),
            "cc-by-sa-4.0" | "cc-by-sa-4" | "ccbysa4" | "ccbysa" => Ok(CCBYSA),
            "cc0-1.0" | "cc0-1" | "cc0" => Ok(CC0),
            "cecill-2.1" | "cecill" => Ok(Cecill),
            "cern-ohl-p-2.0" | "cern-ohl-p" => Ok(CernOHLP),
            "cern-ohl-s-2.0" | "cern-ohl-s" => Ok(CernOHLS),
            "cern-ohl-w-2.0" | "cern-ohl-w" => Ok(CernOHLW),
            "ecl-2.0" | "ecl-2" | "ecl2" | "ecl" => Ok(ECL),
            "epl-1.0" | "epl-1" | "epl1" => Ok(EPL1),
            "epl-2.0" | "epl-2" | "epl2" | "epl" => Ok(EPL2),
            "eupl-1.1" | "eupl1.1" => Ok(EUPL11),
            "eupl-1.2" | "eupl1.2" | "eupl" => Ok(EUPL12),
            "gfdl-1.3" | "gfdl1.3" | "gfdl" => Ok(GFDL),
            "gpl-2.0" | "gpl-2" | "gpl2" | "gplv2" => Ok(GPL2),
            "gpl-3.0" | "gpl-3" | "gpl3" | "gplv3" | "gpl" => Ok(GPL3),
            "isc" => Ok(ISC),
            "lgpl-2.1" | "lgpl-2" | "lgpl2" => Ok(LGPL2),
            "lgpl-3.0" | "lgpl-3" | "lgpl3" | "lgpl" => Ok(LGPL3),
            "lppl-1.3c" | "lppl-1.3" | "lppl1.3" | "lppl" => Ok(LPPL),
            "mit-0" | "mit0" => Ok(MIT0),
            "mit" => Ok(MIT),
            "mpl-2.0" | "mpl-2" | "mpl2" | "mpl" => Ok(MPL),
            "ms-pl" | "mspl" => Ok(MSPL),
            "ms-rl" | "msrl" => Ok(MSRL),
            "mulanpsl-2.0" | "mulanpsl-2" | "mulanpsl2" | "mulanpsl" | "mulan" => Ok(MulanPSL),
            "ncsa" | "uiuc" => Ok(NCSA),
            "odbl-1.0" | "odbl-1" | "odbl1" | "odbl" => Ok(ODbL),
            "ofl-1.1" | "ofl1.1" | "ofl" => Ok(OFL),
            "osl-3.0" | "osl-3" | "osl3" | "osl" => Ok(OSL),
            "postgresql" | "psql" => Ok(PostgreSQL),
            "unlicense" => Ok(Unlicense),
            "upl-1.0" | "upl-1" | "upl1" | "upl" => Ok(UPL),
            "vim" => Ok(Vim),
            "wtfpl" => Ok(WTFPL),
            "zlib" => Ok(Zlib),
            other => Err(format!("Invalid license specifier {other}")),
        }
    }
}

#[derive(Debug)]
enum Subcommand {
    Print,
    Check,
}

#[derive(Debug, Clone)]
struct LicenseInfo {
    name: Option<String>,
    year: Option<String>,
    email: Option<String>,
    project: Option<String>,
    project_url: Option<String>,
}

#[derive(Debug)]
struct Args {
    subcommand: Option<Subcommand>,

    info: LicenseInfo,
    extension: Option<String>,

    licenses: Vec<License>,
}

fn parse_args() -> Args {
    let mut args = pico_args::Arguments::from_env();

    // Parse flags
    let help = args.contains(["-h", "--help"]);
    if help {
        println!("{HELP}");
        process::exit(0);
    }

    let version = args.contains(["-V", "--version"]);
    if version {
        println!("{VERSION}");
        process::exit(0);
    }

    let list = args.contains(["-l", "--list"]);
    if list {
        println!("{VALID_LICENSES}");
        process::exit(0);
    }

    let print = args.contains(["-p", "--print"]);
    let check = args.contains(["-c", "--check"]);

    // Parse possible subcommand
    let subcommand = match (print, check) {
        (false, false) => None,
        (true, false) => Some(Subcommand::Print),
        (false, true) => Some(Subcommand::Check),
        (true, true) => {
            eprintln!(
                "Error: Only one of: '-p, --print' or '-c, --check' can be specified at once"
            );
            eprintln!("{TRY_HELP}");
            process::exit(2);
        }
    };

    // Parse options
    let Ok(extension) = args.opt_value_from_str(["-e", "--extension"]) else {
        eprintln!(
            "Error: option '-e, --extension' requires an argument EXT, but none was supplied"
        );
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let Ok(name) = args.opt_value_from_str(["-n", "--name"]) else {
        eprintln!("Error: option '-n, --name' requires an argument NAME, but none was supplied");
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let Ok(year) = args.opt_value_from_str(["-y", "--year"]) else {
        eprintln!("Error: option '-y, --year' requires an argument YEAR, but none was supplied");
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let Ok(email) = args.opt_value_from_str(["-m", "--email"]) else {
        eprintln!("Error: option '-m, --email' requires an argument EMAIL, but none was supplied");
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let Ok(project) = args.opt_value_from_str(["-j", "--project"]) else {
        eprintln!("Error: option '-j, --project' requires an argument PROJ, but none was supplied");
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let Ok(project_url) = args.opt_value_from_str(["-u", "--url"]) else {
        eprintln!("Error: option '-u, --url' requires an argument URL, but none was supplied");
        eprintln!("{TRY_HELP}");
        process::exit(2);
    };
    let info = LicenseInfo {
        name,
        year,
        email,
        project,
        project_url,
    };

    // Remaining arguments are LICENSEs
    let mut found_invalid_license = false;
    let licenses: Vec<License> = args
        .finish()
        .into_iter()
        .filter_map(
            |os_str| match os_str.to_string_lossy().to_lowercase().as_str().try_into() {
                Ok(license_ident) => Some(license_ident),
                Err(msg) => {
                    found_invalid_license = true;
                    eprintln!("Error: {msg}");
                    None
                }
            },
        )
        .collect();
    if found_invalid_license {
        eprintln!("{TRY_LIST}");
        process::exit(2);
    }
    if licenses.is_empty() {
        eprintln!("Error: At least one valid LICENSE must be specified");
        eprintln!("{TRY_LIST}");
        process::exit(2);
    };

    Args {
        licenses,
        info,
        extension,
        subcommand,
    }
}

fn print_licenses(licenses: Vec<License>, info: LicenseInfo) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    for license in licenses {
        write_license(&mut stdout, license, &info)?;
    }
    Ok(())
}

fn output_licenses(
    licenses: Vec<License>,
    info: LicenseInfo,
    extension: Option<String>,
) -> io::Result<()> {
    if licenses.len() == 1 {
        let license = licenses[0];
        let mut path = PathBuf::from("LICENSE");
        if let Some(ext) = extension {
            path.set_extension(ext);
        }
        let mut file = File::create_new(path)?;
        write_license(&mut file, license, &info)?;
    } else {
        for license in licenses {
            let specifier = license.specifier();
            let mut path = PathBuf::from(format!("LICENSE-{specifier}"));
            if let Some(ext) = &extension {
                path.set_extension(ext);
            }
            let mut file = File::create_new(path)?;
            write_license(&mut file, license, &info)?;
        }
    }
    Ok(())
}

fn write_license<W: Write>(writer: &mut W, license: License, info: &LicenseInfo) -> io::Result<()> {
    writer.write_all(licenses::content(license, info).as_bytes())?;
    Ok(())
}

fn main() {
    let args = parse_args();

    let result = match &args.subcommand {
        Some(Subcommand::Check) => process::exit(0),
        Some(Subcommand::Print) => print_licenses(args.licenses, args.info),
        None => output_licenses(args.licenses, args.info, args.extension),
    };
    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
