# dat2csv

dat2csv is a command line utility written in Rust that takes a data file generated with Data Mover (Oracle PeopleSoft) and generates one csv per each DB table/record that the .dat file contains.

The main goal of this program was to learn Rust.

Usage:
======
rust run <name_of_the_data_file>

e.g. rust run example.dat

Expected output:
================
A csv file for each PeopleSoft Record/DB Table in the .dat file

Notes to the Program Logic:
==========================
Read the dat file as String. 
String is a vector of bytes in UTF-8. 
Chars is an iterable over char representation of these bytes.
Find all EXPORT <table> locations. 
Find all Fields Definition location. 
Find Save them for later iteration. 
Work with slices of the String for each table.
Based on slash structure, get Field Structure, and then the data. 
To parse each string, iterate throw chars with receipes from https://wduquette.github.io/parsing-strings-into-slices/
Line by Line may not the best approach. Try it anyway.


PeopleTools Data Analysis:
==========================
SET VERSION_DAM  8.5:1:0
      
SET ENDIAN LE
SET BASE_LANGUAGE ENG
REM Database: PENGD
REM Started: Mon Apr 25 21:15:03 2022

Find Table
First Export are Tablespaces
From Second EXPORT onward will be PS DB tables

EXPORT <Tabla>.PS_<Tabla> <Query>
<query 2>
/
B(AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA)
/ 2
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
/ 3
A(1900-01-01),A(S0085249),A(N),A(BE - ENGINEERING OFFICE),A(S0085249),A(N),
A(Y),A(1900-01-01),A(9c875610c4fc496499e39741b6541dbc),A( ),A( ),A( ),A(O)
A(RG_SUBTYPE_DN_BRANCH#RDOFI),A(S0009867),A(N),A(N),A(Y),A(N),A(00018)
A(800_ESP),A(00368460),A(Y),
// 4
A(1900-01-01),A(S0010037),A(N),A(BE- FINANCIAL CORE DATA),A(S0010037),A(N),
A(Y),A(1900-01-01),A(9c875610c4fc496499e39741b6541dbc),A( ),A( ),A( ),A(O)
A(RG_SUBTYPE_DN_BRANCH#RDOFI),A(S0002020),A(N),A(N),A(Y),A(N),A(00011)
A(300_ESP),A(00000639),A(Y),
//

Fields
1) Field and data type separated with colons <:>
2) Separated with <~~~>

Actual Data
A(),
Ends with comma <,>


iso8859 is a generated .dat containing codes for utf8 special values
ps_codes: 
        "\"", r"\(", r"\)", "FL", "\\", "FN", "~", "OCICKM", "MCLP", "MCLP", "MCLP", "MCLP",
        "MCLP", "MCLP", "MCLP", "MCLP", "MFKA", "MCLP", "MFJC", "MFLN", "`", "MCLP", "MCLP",
        "MCLP", "MCLP", "MCLP", "MCLP", "MCLP", "MFKB", "MCLP", "MFJD", "MFLO", "MFLI", "MCKB",
        "MCKC", "MCKD", "MCLP", "MCKF", "MCLP", "MCKH", "MCLP", "MCKJ", "MCKK", "MCKL", "MCKM",
        "MCKN", "MCKO", "MCKP", "MCLA", "MCLB", "MCLC", "MCLD", "MCLP", "MCLF", "MCLG", "MCLH",
        "MCLP", "MCLJ", "MCLK", "MCLL", "MCLP", "MCLP", "MCLP", "MCLP", "MDIA", "MDIB", "MDIC",
        "MDID", "MDIE", "MDIF", "MDIG", "MDIH", "MDII", "MDIJ", "MDIK", "MDIL", "MDIM", "MDIN",
        "MDIO", "MDIP", "MDJA", "MDJB", "MDJC", "MDJD", "MDJE", "MDJF", "MDJG", "MDJH", "MDJI",
        "MDJJ", "MDJK", "MDJL", "MDJM", "MDJN", "MDJO", "MDJP", "MDKA", "MDKB", "MDKC", "MDKD",
        "MDKE", "MDKF", "MDKG", "MDKH", "MDKI", "MDKJ", "MDKK", "MDKL", "MDKM", "MDKN", "MDKO",
        "MDKP", "MDLA", "MDLB", "MDLC", "MDLD", "MDLE", "MDLF", "MDLG", "MDLH", "MDLI", "MDLJ",
        "MDLK", "MDLL", "MDLM", "MDLN", "MDLO", "MDLP",
iso_list: 
        '"', '(', ')', '[', '\\', ']', '~', '€', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹',
        'Œ', 'Ž', '‘', '’', '“', '”', '•', '–', '—', '™', 'š', '›', 'œ', 'ž', 'Ÿ', '¡', '¢', '£',
        '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', '­', '®', '¯', '°', '±', '²', '³', '´', 'µ',
        '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç',
        'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', 'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù',
        'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', 'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë',
        'ì', 'í', 'î', 'ï', 'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý',
        'þ', 'ÿ',
