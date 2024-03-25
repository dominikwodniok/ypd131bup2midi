# Introduction
This project allows to extract MIDI data from BUP data dump files of the old YDP-131 digital piano. While the YDP-131 allows to record performances which are stored as MIDI data, it is lacking software to download that data. Contrary to official statements the vendor provided software only allows to create backups of internal data which is stored in BUP.

Example usage:

$ ./ypd131bup2midi BUPFILE

The program outputs a file with the name of the input file but with a MIDI extension.

**Warning:** I'm not a Rustacean and probably commited many rustrocities. This software was developed as a favor for a friend and meant as a first contact with the Rust programming language.

## Changelog
- 2024-03-25 Published on GitHub
- 2018-11-xx Initial version