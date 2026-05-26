#!/bin/bash


CLEAN_STATS_FORMAT=$'
--------\n
STATS
 Real       : %E
 User       : %U
 Sys        : %S
 CPU        : %P
 Peak RAM   : %M KB
 Avg  RAM   : %K KB
 Page size  : %Z bytes
 Major pf   : %F
 Minor pf   : %R
 FS Inputs  : %I
 FS Outputs : %O
 Vol SW     : %c
 Invol SW   : %w
 Exit       : %x
 
COMMAND
 %C
'

/usr/bin/time -f "$CLEAN_STATS_FORMAT" "$@"