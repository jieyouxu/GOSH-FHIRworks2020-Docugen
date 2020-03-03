# GOSH-FHIRworks2020-Docugen

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/42a7c1f4d5314c7f9b44bca857292ac6)](https://app.codacy.com/manual/jieyouxu/GOSH-FHIRworks2020-Docugen?utm_source=github.com&utm_medium=referral&utm_content=jieyouxu/GOSH-FHIRworks2020-Docugen&utm_campaign=Badge_Grade_Dashboard)
[![codecov](https://codecov.io/gh/jieyouxu/GOSH-FHIRworks2020-Docugen/branch/master/graph/badge.svg)](https://codecov.io/gh/jieyouxu/GOSH-FHIRworks2020-Docugen)
![GitHub language count](https://img.shields.io/github/languages/count/jieyouxu/GOSH-FHIRworks2020-Docugen)
![GitHub top language](https://img.shields.io/github/languages/top/jieyouxu/GOSH-FHIRworks2020-Docugen?color=orange)
[![Build Status](https://travis-ci.com/jieyouxu/GOSH-FHIRworks2020-Docugen.svg?branch=master)](https://travis-ci.com/jieyouxu/GOSH-FHIRworks2020-Docugen)
![Crates.io](https://img.shields.io/crates/v/gosh_fhirworks2020_docugen)
![GitHub deployments](https://img.shields.io/github/deployments/jieyouxu/GOSH-FHIRworks2020-Docugen/github-pages?label=documentation&logo=GitHub)

A document generation tool `Docugen` for the GOSH Drive FHIRworks API.

## Submodules

```text
.
├── FHIRworks_2020                    // (intermediate web API submodule)
└── gosh_fhirworks2020_docugen        // (this tool)
```

The submodule `FHIRworks_2020` contains the intermediate web API for
interpolation between the backend FHIRworks API and the front-end `Docugen`
tool.

Run the `dotnet-azure-fhir-web-api` and bind it to `https://localhost:5001` so
the front-end tooling can utilize this intermediate endpoint.

## Documentation

Deployed at [docugen](https://jieyouxu.github.io/GOSH-FHIRworks2020-Docugen/docugen/index.html).

## Configuration

To configure the tooling, copy the example `config.example.toml` configuration
file.

```bash
cd gosh_fhirworks2020_docugen
cp config.example.toml config.toml
```
