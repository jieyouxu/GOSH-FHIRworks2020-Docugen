# GOSH-FHIRworks2020-Docugen

A document generation tool `Docugen` for the GOSH Drive FHIRworks API.

## Submodules

The submodule `FHIRworks_2020` contains the intermediate web API for
interpolation between the backend FHIRworks API and the front-end `Docugen`
tool.

Run the `dotnet-azure-fhir-web-api` and bind it to `https://localhost:5001` so
the front-end tooling can utilize this intermediate endpoint.
