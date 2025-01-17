The aim of this project is to create a querying tool using particle data group (PDG) released database without using their API.
It is possible because the size of the database is very small, ~few MB.

This project aims to achieve the following query:
Querying by name/name alias/pdgid/node id
- Using pdgQuery e, pdgQuery tau+, pdgQuery mu, pdgQuery 22, pdgQuery S003
- Returning related informations including name, quantum numbers, parties, mass, decay channels and many other physical parameters.
- It aims to provide very rich information including the error of latest measurement given by the PDG group.

Querying by decays
- pdgQuery ? -> ee, returns particles that has dielectron decay
- pdgQuery ? -> e ? ?, returns particle decay channels that decay into one electron and two other particles
- pdgQuery ? -> e nu_e ?* returns particle decays that contains electron and electron neutrino

Maybe: Querying by physical properties
- Search particle that match given physical properties, 

