# dat2csv

PeopleSoft Data Mover enables you to:

- Transfer application data between PeopleSoft databases.
- Move PeopleSoft databases across operating systems and database platforms.
- Execute Structured Query Language (SQL) statements against any PeopleSoft database, regardless of the underlying operating system or database platform.
- Control database security and access.
- Create, edit, and run scripts.

These scripts may include any combination of SQL commands and PeopleSoft Data Mover commands for exporting and importing data.

dat2csv is a command line utility written in Rust that takes a data file generated with Data Mover and generates one csv per each DB table/record that the .dat file contains. This has been tested with PeopleTools 8.5{2-8}. 

The main goal of this program was to learn Rust.

## Usage:
cargo run <name_of_the_data_file>

e.g. cargo run example.dat

## Expected output:
A csv file for each PeopleSoft Record/DB Table in the .dat file

## Notes to the Program Logic:
| Step | Action |
|-----:|-----------|
|     1| Read the dat file as String.|
|     2| String is a vector of bytes in UTF-8.    |
|     3| Chars is an iterable over char representation of these bytes.       |
|     4| Find all EXPORT \<table\> locations.       |
|     5| Find all Fields Definition location.       |
|     6| Find Save them for later iteration.        |
|     7| Work with slices of the String for each table.       |
|     8| Based on slash structure, get Field Structure, and then the data.       |
|     9| To parse each string, iterate throw chars with receipes from https://wduquette.github.io/parsing-strings-into-slices/|
|     10| Line by Line may not the best approach. Try it anyway.|

## PeopleTools Data Analysis:

- See in /assets/Example.dat how a data file is structured. EXPORT keyboard indicates the DB Table/Record, and lines beginning with slashes mark the different Field block and Data blocks.

EXPORT \<Table\>.PS_\<Table\> \<Query\>

\/
B(AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA)
\/ 2
Z_FECHA_CAR:CHAR(10)~~~Z_WD_ORGANIZA_ID:CHAR(11)~~~
Z_WD_INCLUDE_ORGAN:CHAR(1)~~~Z_WD_SUPVISOR_NAME:CHAR(70)~~~
Z_WD_ORGANIZA_CODE:CHAR(11)~~~Z_WD_INCLUDE_CODE_:CHAR(1)~~~
Z_WD_ACTIVE:CHAR(1)~~~Z_WD_AVAILAB_DT:CHAR(10)~~~
Z_WD_VISIBILITY:CHAR(70)~~~Z_WD_EXTR_URL_RF:CHAR(200)~~~
Z_WD_INTEGRATION_S:CHAR(30)~~~Z_WD_INTEGRATION_I:CHAR(30)~~~
Z_WD_SUBTYPE:CHAR(50)~~~Z_WD_OPRGANIZA_SUP:CHAR(11)~~~
Z_WD_INCLUDE_LEADE:CHAR(1)~~~Z_WD_JOB_MANAGEMEN:CHAR(1)~~~
Z_WD_POS_MANAGEMEN:CHAR(1)~~~Z_WD_HIRING_FREEZE:CHAR(1)~~~
Z_WD_PRI_LOCATION:CHAR(55)~~~Z_WD_REPORTS_TO:CHAR(15)~~~
Z_WD_MAN_COD_SUP:CHAR(1)~~~
\/ 3
A(1900-01-01),A(S0085249),A(N),A(BE - ENGINEERING OFFICE),A(S0085249),A(N),
A(Y),A(1900-01-01),A(9c875610c4fc496499e39741b6541dbc),A( ),A( ),A( ),A(O)
A(RG_SUBTYPE_DN_BRANCH#RDOFI),A(S0009867),A(N),A(N),A(Y),A(N),A(00018)
A(800_ESP),A(00368460),A(Y),
\/\/ 4
A(1900-01-01),A(S0010037),A(N),A(BE- FINANCIAL CORE DATA),A(S0010037),A(N),
A(Y),A(1900-01-01),A(9c875610c4fc496499e39741b6541dbc),A( ),A( ),A( ),A(O)
A(RG_SUBTYPE_DN_BRANCH#RDOFI),A(S0002020),A(N),A(N),A(Y),A(N),A(00011)
A(300_ESP),A(00000639),A(Y),
\/\/

Fields
1) Field and data type separated with colons <:>
2) Separated with <~~~>

Actual Data
A(),
Ends with comma <,>

- The file /assets/iso8859.dat is a generated .dat containing ps codes corresponding to utf8 special values.
