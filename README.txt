Algorithm:

Read the file as String. String is a vector of bytes in UTF-8. Chars is an iterable over char representation of these bytes.
Find all EXPORT <table> locations. Find all Fields Definition location. Find Save them for later iteration. Work with slices of the String for each table.
Based on slash structure, get Field Structure, and then the data. 
To parse each string, iterate throw chars with receipes from https://wduquette.github.io/parsing-strings-into-slices/

Line by Line may not the best approach. Try it anyway.


PeopleTools Data
SET VERSION_DAM  8.5:1:0
      
SET ENDIAN LE
SET BASE_LANGUAGE ENG
REM Database: PENGD
REM Started: Mon Apr 25 21:15:03 2022

Encontrar Tabla
Primer Export son Tablespces
A Partir del segundo EXPORT son tablas

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

Encontrar Campos
1) Campo y tipo dato separado por :
2) Separados por ~~~

Datos
A(),
Termina con ,



The BufReader<R> struct adds buffering to any reader.
It can be excessively inefficient to work directly with a Read instance. For example, every call to read on TcpStream results in a system call. A BufReader<R> performs large, infrequent reads on the underlying Read and maintains an in-memory buffer of the results.
BufReader<R> can improve the speed of programs that make small and repeated read calls to the same file or network socket. It does not help when reading very large amounts at once, or reading just one or a few times. It also provides no advantage when reading from a source that is already in memory, like a Vec<u8>.
When the BufReader<R> is dropped, the contents of its buffer will be discarded. Creating multiple instances of a BufReader<R> on the same stream can cause data loss. Reading from the underlying reader after unwrapping the BufReader<R> with BufReader::into_inner can also cause data loss.
