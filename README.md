## License

This project is licensed under the [MIT License](LICENSE). See the `LICENSE` file for more details.


## pdgQuery
Disclaimer: this project is a personal project that aims to learn rust and create some tools for fun under MIT License.

The goal of this project is to create a querying tool using particle data group (PDG) released database without using their API: https://pdg.lbl.gov/2024/api/index.html
It is possible because the size of the database is very small, ~few MB.

This project aims to achieve the following query:

Querying by name/name alias/pdgid/node id
- Using pdgQuery e, pdgQuery tau+, pdgQuery mu, pdgQuery 22, pdgQuery S003
- Returning related information including name, quantum numbers, parties, mass, decay channels and many other physical parameters.
- It aims to provide very rich information including the error of the latest measurement given by the PDG group.

Querying by decays, Query characters has been changed due to conflict with special treatment on ? and > in terminal.
- pdgQuery "? -> e+e-" returns particles that has dielectron decay ✔️
- pdgQuery "? -> e ? ?" returns particle decay channels that decay into one electron and two other particles ✔️
- pdgQuery "? -> e nu_e ?*" returns particle decays that contains electron and electron neutrino ✔️

Currently the package work as expected, however, due to the inconsistent formatting in the database user may not find all the relevant results they wanted.
This will be the next objective to extend the functionality.

Maybe: Querying by physical properties
- Search particle that match given physical properties, 
- Draw some level of Feynman diagrams with aids from other tools

## Installation
To use this repository, you need to define a `.env` file, and set the `PDGDB_PATH` variable to the database location.
Install rust compiler with `curl https://sh.rustup.rs -sSf | sh`, and run `cargo install --path .` to install the binary. Test if you can run `pdgQuery e-`.