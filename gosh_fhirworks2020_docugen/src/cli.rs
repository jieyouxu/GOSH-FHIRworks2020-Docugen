use clap::{App, AppSettings, Arg};

/// We devise a CLI interface for docugen.
///
/// We get data from an API endpoint, e.g. `/api/Patients`.
///
/// We read a template file from the filesystem, e.g. from
/// `./templates/patient_birthdates_list.template`.
///
/// We save the output to either `stdout` or a user-specified output file.
pub fn cli<'a, 'b>() -> App<'a, 'b> {
    let config_arg = Arg::with_name("config")
        .short("c")
        .long("config")
        .value_name("FILE")
        .help("Sets config from custom file.")
        .takes_value(true);

    let endpoint_arg = Arg::with_name("ENDPOINT")
        .help("Select the endpoint to use, e.g. `/api/Patient`. Configure the IP address and port in the configuration file.")
        .required(true)
        .index(1);

    let template_arg = Arg::with_name("TEMPLATE")
        .help("Select the template to fill the data fetched from the endpoint.")
        .required(true)
        .index(2);

    let verbosity_arg = Arg::with_name("v")
        .short("v")
        .multiple(true)
        .help("Sets the level of logging verbosity.");

    App::new("FHIRworks2020 docugen")
            .version("0.1.0")
            .author("Jieyou Xu (Joe) <jieyou.xu.18@ucl.ac.uk>")
            .about("Small CLI tool to fetch data from a FHIR API endpoint and fill out a document template.")
            .setting(AppSettings::ColoredHelp)
            .arg(&config_arg)
            .arg(&endpoint_arg)
            .arg(&template_arg)
            .arg(&verbosity_arg)
}
