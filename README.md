Safe Rust bindings for [RTKLIB](https://github.com/rtklibexplorer/RTKLIB).

The unsafe bindings in `rtklib-sys` expose the full RTKLIB API. The safe wrappers
in `rtklib-ffi` cover a subset - see the coverage tables below for current status.

## Features

| Feature | Description |
|---------|-------------|
| `conv` | RINEX and other file format conversion; implies `receivers` because `convrnx` references `init_raw` unconditionally at link time |
| `gis` | GIS data support |
| `hifitime` | Conversions between `GpsTime` and `hifitime::Epoch` |
| `net` | Network streaming |
| `ppk` | Post-processed kinematic positioning via `postpos()` and solution I/O |
| `receivers` | All supported hardware receiver decoders: BINEX, Hemisphere Crescent, Javad/Topcon, NovAtel OEM, NVS, Septentrio SBF, SkyTraq, Swift Navigation SBP, Trimble RT17, u-blox UBX, and Unicore |
| `rtcm` | RTCM3 message decoding |
| `strum` | `Display` for enums via the `strum` crate |
| `tle` | TLE satellite tracking |

## Quick Start

```toml
[dependencies]
rtklib-ffi = { version = "0.3", features = ["ppk"] }
```

```rust
use rtklib_ffi::ppk::{FilOpt, PrcOpt, SolOpt, postpos};

let popt = PrcOpt::kinematic();
let sopt = SolOpt::default();
let fopt = FilOpt::default();

postpos(
    "rover.obs", "base.obs",
    &["nav.nav"], "output.pos",
    &popt, &sopt, &fopt,
).unwrap();
```

## RTKLIB Coverage

`[x]` = safe wrapper, `[ ]` = unsafe bindings only.

---

### Core

Always compiled.

**`rtkcmn.c`**

Coordinate transforms, time conversions, satellite utilities, and linear algebra.

- [x] `ecef2pos`: ECEF to geodetic
- [x] `pos2ecef`: geodetic to ECEF
- [x] `satno`: constellation + PRN to satellite number
- [x] `gpst2time`: GPS week/TOW to `gtime_t`
- [x] `getbitu`: unsigned bit extraction
- [ ] `ecef2enu`: ECEF to local ENU
- [ ] `enu2ecef`: local ENU to ECEF
- [ ] `xyz2enu`: rotation matrix ECEF to ENU
- [ ] `covecef`: covariance ECEF to ENU
- [ ] `covenu`: covariance ENU to ECEF
- [ ] `geodist`: geometric distance between satellite and receiver
- [ ] `satazel`: satellite azimuth/elevation
- [ ] `satexclude`: check satellite exclusion
- [ ] `satsys`: satellite system from satellite number
- [ ] `satid2no`: satellite ID string to number
- [ ] `satno2id`: satellite number to ID string
- [ ] `ionmodel`: Klobuchar ionosphere model
- [ ] `tropmodel`: Saastamoinen troposphere model
- [ ] `ionmapf`: ionosphere mapping function
- [ ] `tropmapf`: troposphere mapping function
- [ ] `antmodel`: antenna phase center offset model
- [ ] `antmodel_s`: satellite antenna phase center offset model
- [ ] `obs2code`: observation code string to number
- [ ] `code2obs`: code number to observation code string
- [ ] `code2freq`: code number to carrier frequency
- [ ] `code2idx`: code number to frequency index
- [ ] `sat2freq`: satellite + code to carrier frequency
- [ ] `seliflc`: select iono-free linear combination
- [ ] `dops`: dilution of precision
- [ ] `epoch2time`: calendar epoch to `gtime_t`
- [ ] `time2epoch`: `gtime_t` to calendar epoch
- [ ] `time2epoch_n`: `gtime_t` to calendar epoch with nanoseconds
- [ ] `time2gpst`: `gtime_t` to GPS week/TOW
- [ ] `gpst2utc`: GPS time to UTC
- [ ] `utc2gpst`: UTC to GPS time
- [ ] `gpst2bdt`: GPS time to BeiDou time
- [ ] `bdt2gpst`: BeiDou time to GPS time
- [ ] `bdt2time`: BeiDou week/TOW to `gtime_t`
- [ ] `time2bdt`: `gtime_t` to BeiDou week/TOW
- [ ] `gst2time`: Galileo week/TOW to `gtime_t`
- [ ] `time2gst`: `gtime_t` to Galileo week/TOW
- [ ] `timeadd`: add seconds to `gtime_t`
- [ ] `timediff`: difference of two `gtime_t` values
- [ ] `time2str`: `gtime_t` to string
- [ ] `str2time`: string to `gtime_t`
- [ ] `time2doy`: `gtime_t` to day of year
- [ ] `adjgpsweek`: adjust GPS week rollover
- [ ] `screent`: time interval screen
- [ ] `readpcv`: read antenna parameter file
- [ ] `searchpcv`: search antenna parameters
- [ ] `readerp`: read earth rotation parameters
- [ ] `geterp`: get earth rotation parameter at time
- [ ] `readblq`: read ocean tide loading parameters
- [ ] `readpos`: read reference station positions
- [ ] `getstapos`: get station position
- [ ] `readnav`: read RINEX navigation data
- [ ] `savenav`: save navigation data to file
- [ ] `freeobs`: free observation data
- [ ] `freenav`: free navigation data
- [ ] `sortobs`: sort and remove duplicate observations
- [ ] `uniqnav`: remove duplicate navigation data
- [ ] `lsq`: least-squares estimation
- [ ] `filter`: Kalman filter update
- [ ] `smoother`: Rauch-Tung-Striebel smoother
- [ ] `matmul` / `matinv` / `solve`: matrix operations
- [ ] `mat` / `imat` / `zeros` / `eye`: matrix allocation
- [ ] `dot` / `dot2` / `dot3` / `norm` / `cross3` / `normv3`: vector operations
- [ ] `str2num` / `deg2dms` / `dms2deg`: string and angle utilities
- [ ] `rtk_crc24q` / `rtk_crc16` / `rtk_crc32`: CRC functions
- [ ] `setbitu` / `setbits` / `getbits`: bit manipulation
- [ ] `decode_word`: GPS navigation word decode
- [ ] `rtk_uncompress`: RTCM3 data decompression
- [ ] `sunmoonpos`: sun/moon position in ECEF
- [ ] `eci2ecef`: ECI to ECEF rotation
- [ ] `utc2gmst`: UTC to Greenwich Mean Sidereal Time
- [ ] `ionppp`: ionosphere pierce point
- [ ] `testsnr`: test signal-to-noise ratio
- [ ] `setcodepri` / `getcodepri`: code priority
- [ ] `showmsg`: progress message callback
- [ ] `timeget` / `timeset` / `timereset`: system time
- [ ] `tickget` / `sleepms`: tick counter and sleep
- [ ] `execcmd` / `expath` / `createdir`: OS utilities
- [ ] `reppath` / `reppaths`: path substitution
- [ ] `read_leaps`: read leap second file

**`trace.c`**

- [ ] `traceopen` / `traceclose`: open/close trace file
- [ ] `tracelevel` / `gettracelevel`: set/get trace level
- [ ] `trace_impl` / `tracet_impl` / `traceb_impl`: trace output
- [ ] `tracemat_impl` / `traceobs_impl` / `tracenav_impl`: trace data structures
- [ ] `tracegnav_impl` / `tracehnav_impl` / `tracepeph_impl` / `tracepclk_impl`: trace nav data

**`geoid.c`**

- [ ] `opengeoid`: open geoid data file
- [ ] `closegeoid`: close geoid data file
- [ ] `geoidh`: geoid height at position

**`datum.c`**

- [ ] `loaddatump`: load datum parameters
- [ ] `tokyo2jgd`: Tokyo datum to JGD2000
- [ ] `jgd2tokyo`: JGD2000 to Tokyo datum

---

### PPK - `ppk` feature

**`postpos.c`**

- [x] `postpos`: full PPK post-processing pipeline

**`solution.c`**

- [x] `readsol`: read solution files into buffer
- [x] `readsolt`: read solution files with time window
- [x] `freesolbuf`: free solution buffer
- [x] `outsolheads`: write solution file header
- [x] `outsols`: write one solution record
- [ ] `initsolbuf`: initialize solution buffer
- [ ] `addsol`: append a solution record
- [ ] `getsol`: get solution record by time
- [ ] `inputsol`: decode one solution record from byte stream
- [ ] `outsol`: write solution to stream
- [ ] `outsolex`: write extended solution to stream
- [ ] `outsolexs`: write extended solution as string
- [ ] `outsolhead`: write solution header to stream
- [ ] `outprcopt`: write processing options to stream
- [ ] `outprcopts`: write processing options as string
- [ ] `outnmea_gga`: output NMEA GGA sentence
- [ ] `outnmea_rmc`: output NMEA RMC sentence
- [ ] `outnmea_gsa`: output NMEA GSA sentence
- [ ] `outnmea_gsv`: output NMEA GSV sentence
- [ ] `outnmea_gst`: output NMEA GST sentence
- [ ] `readsolstat`: read solution status file
- [ ] `readsolstatt`: read solution status file with time window
- [ ] `freesolstatbuf`: free solution status buffer

**`rtkpos.c`**

- [ ] `rtkinit`: initialize RTK control struct
- [ ] `rtkfree`: free RTK control struct
- [ ] `rtkpos`: single-epoch RTK positioning
- [ ] `rtkopenstat`: open RTK status file
- [ ] `rtkclosestat`: close RTK status file
- [ ] `rtkoutstat`: output RTK status record

**`pntpos.c`**

- [ ] `pntpos`: single-point positioning
- [ ] `ionocorr`: ionosphere correction
- [ ] `tropcorr`: troposphere correction

**`rinex.c`**

- [ ] `init_rnxctr`: initialize RINEX controller
- [ ] `free_rnxctr`: free RINEX controller
- [ ] `open_rnxctr`: open RINEX file via controller
- [ ] `input_rnxctr`: read one epoch via controller
- [ ] `readrnx`: read RINEX obs/nav file
- [ ] `readrnxt`: read RINEX file with time window
- [ ] `readrnxc`: read RINEX file via controller
- [ ] `outrnxobsh` / `outrnxobsb`: write RINEX observation header/body
- [ ] `outrnxnavh` / `outrnxnavb`: write RINEX GPS nav header/body
- [ ] `outrnxgnavh` / `outrnxgnavb`: write RINEX GLONASS nav header/body
- [ ] `outrnxhnavh` / `outrnxhnavb`: write RINEX SBAS nav header/body
- [ ] `outrnxqnavh`: write RINEX QZSS nav header
- [ ] `outrnxlnavh`: write RINEX Galileo nav header
- [ ] `outrnxcnavh`: write RINEX BeiDou nav header
- [ ] `outrnxinavh`: write RINEX NavIC nav header
- [ ] `rnxcomment`: add comment to RINEX header

**`ephemeris.c`**

- [ ] `eph2pos`: broadcast GPS/Galileo/BeiDou satellite position
- [ ] `eph2clk`: broadcast satellite clock
- [ ] `geph2pos`: GLONASS satellite position
- [ ] `geph2clk`: GLONASS satellite clock
- [ ] `seph2pos`: SBAS satellite position
- [ ] `seph2clk`: SBAS satellite clock
- [ ] `satpos`: satellite position and clock
- [ ] `satposs`: satellite positions for all observations
- [ ] `alm2pos`: almanac satellite position
- [ ] `getseleph` / `setseleph`: ephemeris selection

**`preceph.c`**

- [ ] `readsp3`: read SP3 precise ephemeris file
- [ ] `peph2pos`: precise satellite position and clock
- [ ] `pephclk`: precise satellite clock
- [ ] `readdcb`: read differential code bias file
- [ ] `readsap`: read satellite antenna parameters
- [ ] `satantoff`: satellite antenna phase center offset
- [ ] `code2bias`: look up signal code bias

**`lambda.c`**

- [ ] `lambda`: LAMBDA ambiguity resolution
- [ ] `lambda_reduction`: LAMBDA decorrelation
- [ ] `lambda_search`: LAMBDA integer search

**`ionex.c`**

- [ ] `readtec`: read IONEX TEC grid file
- [ ] `iontec`: ionosphere delay from TEC grid

**`sbas.c`**

- [ ] `sbsdecodemsg`: decode SBAS message
- [ ] `sbsupdatecorr`: update SBAS corrections
- [ ] `sbssatcorr`: apply SBAS satellite corrections
- [ ] `sbsioncorr`: SBAS ionosphere correction
- [ ] `sbstropcorr`: SBAS troposphere correction
- [ ] `sbsreadmsg`: read SBAS message log
- [ ] `sbsreadmsgt`: read SBAS message log with time window
- [ ] `sbsoutmsg`: write SBAS message to stream

**`options.c`**

- [ ] `loadopts`: load processing options from file
- [ ] `saveopts`: save processing options to file
- [ ] `getsysopts` / `setsysopts`: get/set global system options
- [ ] `resetsysopts`: reset to defaults
- [ ] `searchopt`: search option by name
- [ ] `opt2str` / `opt2buf` / `str2opt`: option string conversion

**`ppp.c`**

- [ ] `pppos`: PPP positioning
- [ ] `pppnx`: PPP state vector size
- [ ] `pppoutstat`: output PPP status
- [ ] `yaw_angle`: satellite yaw angle

**`ppp_ar.c`**

- [ ] `ppp_ar`: PPP ambiguity resolution

**`tides.c`**

- [ ] `tidedisp`: tidal displacement

---

### RTCM - `ppk` or `rtcm` feature

**`rtcm.c`**

- [x] `init_rtcm`: initialize RTCM control struct
- [x] `free_rtcm`: free RTCM control struct
- [x] `input_rtcm3`: decode RTCM3 message byte by byte
- [ ] `input_rtcm2`: decode RTCM2 message byte by byte
- [ ] `input_rtcm3f`: decode RTCM3 from file
- [ ] `input_rtcm2f`: decode RTCM2 from file
- [ ] `gen_rtcm2`: generate RTCM2 message
- [ ] `gen_rtcm3`: generate RTCM3 message

**`rtcm2.c`**

Called internally by `rtcm.c`. Not intended for direct use.

**`rtcm3.c`**

Called internally by `rtcm.c`. Not intended for direct use.

**`rtcm3e.c`**

Called internally by `rtcm.c`. Not intended for direct use.

---

### Raw Receiver Decoding - `receivers` feature

All receiver format files are compiled together. `rcvraw.c` calls init, free,
and input functions for every supported format unconditionally, so all format
files must be present in the same link unit.

**`rcvraw.c`**

Generic frame decoders used by all receiver-specific decoders.

- [x] `init_raw`: initialize raw receiver control struct
- [x] `free_raw`: free raw receiver control struct
- [ ] `input_raw`: decode one byte of raw receiver data
- [ ] `input_rawf`: decode raw receiver data from file
- [ ] `decode_frame`: decode GPS navigation frame
- [ ] `decode_glostr`: decode GLONASS navigation string
- [ ] `test_glostr`: test GLONASS string parity
- [ ] `decode_bds_d1` / `decode_bds_d2`: decode BeiDou D1/D2 navigation messages
- [ ] `decode_gal_fnav` / `decode_gal_inav`: decode Galileo F/NAV and I/NAV messages
- [ ] `decode_irn_nav`: decode NavIC navigation message

**`rcv/binex.c`**

- [ ] `input_bnx` / `input_bnxf`: BINEX decoder

~~**`rcv/comnav.c`**~~ - uses APIs removed upstream; excluded until updated

- ~~[ ] `input_cnav` / `input_cnavf`: ComNav decoder~~

**`rcv/crescent.c`**

- [ ] `input_cres` / `input_cresf`: Hemisphere Crescent decoder

**`rcv/javad.c`**

- [ ] `input_javad` / `input_javadf`: Javad/Topcon decoder

**`rcv/novatel.c`**

- [ ] `input_oem4` / `input_oem4f`: NovAtel OEM4/6/7 decoder
- [ ] `input_oem3` / `input_oem3f`: NovAtel OEM3 decoder

**`rcv/nvs.c`**

- [ ] `input_nvs` / `input_nvsf`: NVS decoder
- [ ] `gen_nvs`: generate NVS command

**`rcv/rt17.c`**

- [ ] `input_rt17` / `input_rt17f`: Trimble RT17 decoder

**`rcv/septentrio.c`**

- [x] `init_sbf` / `free_sbf`: initialize/free Septentrio SBF struct
- [x] `input_sbf` / `input_sbff`: Septentrio SBF decoder

**`rcv/skytraq.c`**

- [ ] `input_stq` / `input_stqf`: SkyTraq decoder
- [ ] `gen_stq`: generate SkyTraq command

**`rcv/swiftnav.c`**

- [ ] `input_sbp` / `input_sbpf` / `input_sbpjsonf`: Swift Navigation SBP decoder

~~**`rcv/tersus.c`**~~ - uses APIs removed upstream; excluded until updated

- ~~[ ] `input_tersus` / `input_tersusf`: Tersus decoder~~

**`rcv/ublox.c`**

- [ ] `input_ubx` / `input_ubxf`: u-blox UBX decoder
- [ ] `gen_ubx`: generate u-blox command

**`rcv/unicore.c`**

- [ ] `input_unicore` / `input_unicoref`: Unicore decoder

---

### File Format Conversion - `conv` feature

**`convrnx.c`**

- [ ] `convrnx`: convert receiver raw data to RINEX

**`convkml.c`**

- [ ] `convkml`: convert solution to KML
- [ ] `convcsv`: convert solution to CSV

**`convgpx.c`**

- [ ] `convgpx`: convert solution to GPX

---

### GIS - `gis` feature

**`gis.c`**

- [ ] `gis_read`: read GIS data file
- [ ] `gis_free`: free GIS data

---

### Streaming - `net` feature

**`stream.c`**

- [ ] `strinit`: initialize stream
- [ ] `stropen` / `strclose`: open/close stream
- [ ] `strread` / `strwrite`: read/write stream
- [ ] `strsync`: synchronize two streams
- [ ] `strstat` / `strstatx`: stream status
- [ ] `strsum`: stream statistics
- [ ] `strgettime`: stream time tag
- [ ] `strsendnmea` / `strsendcmd`: send NMEA or command to stream
- [ ] `strsetopt`: set stream options
- [ ] `strsetdir` / `strsetproxy` / `strsettimeout`: stream configuration
- [ ] `strinitcom`: initialize COM port

**`streamsvr.c`**

- [ ] `strsvrinit`: initialize stream server
- [ ] `strsvrstart` / `strsvrstop`: start/stop stream server
- [ ] `strsvrstat`: stream server status
- [ ] `strsvrpeek`: peek stream server buffer
- [ ] `strconvnew` / `strconvfree`: create/free stream converter

**`rtksvr.c`**

- [ ] `rtksvrinit`: initialize RTK server
- [ ] `rtksvrstart` / `rtksvrstop`: start/stop RTK server
- [ ] `rtksvrlock` / `rtksvrunlock`: lock/unlock RTK server
- [ ] `rtksvropenstr` / `rtksvrclosestr`: manage server streams
- [ ] `rtksvrstat` / `rtksvrsstat` / `rtksvrostat`: server status
- [ ] `rtksvrmark`: mark event in RTK server
- [ ] `rtksvrfree`: free RTK server

**`download.c`**

- [ ] `dl_readurls`: read download URL list
- [ ] `dl_readstas`: read station list
- [ ] `dl_exec`: execute downloads
- [ ] `dl_test`: test download URL
- [ ] `execcmd_to`: execute command with timeout

---

### TLE - `tle` feature

**`tle.c`**

- [ ] `tle_read`: read TLE file
- [ ] `tle_name_read`: read TLE file indexed by satellite name
- [ ] `tle_pos`: satellite position from TLE

---

### SOFA

**`sofa.c`**

IAU SOFA routines for high-accuracy sun/moon position. Only compiled when the
`SUNPOS_SOFA` or `MOONPOS_SOFA` macros are defined; this build does not set them,
so `sofa.c` is not a compilation unit here. The lower-accuracy built-in paths in
`rtkcmn.c` are used instead.
