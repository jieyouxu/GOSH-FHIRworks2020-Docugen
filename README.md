# GOSH-FHIRworks2020-Docugen

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/42a7c1f4d5314c7f9b44bca857292ac6)](https://app.codacy.com/manual/jieyouxu/GOSH-FHIRworks2020-Docugen?utm_source=github.com&utm_medium=referral&utm_content=jieyouxu/GOSH-FHIRworks2020-Docugen&utm_campaign=Badge_Grade_Dashboard)
[![codecov](https://codecov.io/gh/jieyouxu/GOSH-FHIRworks2020-Docugen/branch/master/graph/badge.svg)](https://codecov.io/gh/jieyouxu/GOSH-FHIRworks2020-Docugen)

A document generation tool `Docugen` for the GOSH Drive FHIRworks API.

## Submodules

```
.
├── FHIRworks_2020                    // (intermediate web API submodule)
└── gosh_fhirworks2020_docugen        // (this tool)
```

The submodule `FHIRworks_2020` contains the intermediate web API for
interpolation between the backend FHIRworks API and the front-end `Docugen`
tool.

Run the `dotnet-azure-fhir-web-api` and bind it to `https://localhost:5001` so
the front-end tooling can utilize this intermediate endpoint.

## Configuration

To configure the tooling, copy the example `config.example.toml` configuration
file.

```bash
$ cd gosh_fhirworks2020_docugen
$ cp config.example.toml config.toml
```
