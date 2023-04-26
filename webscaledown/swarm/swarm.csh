#!/bin/tcsh

rm -f bad good
touch bad
touch good

set me=`readlink -f $0`
set me=$me:h
cd "$me"

set pids=()

if( $# == 0 ) then
    exit 2
endif

if( $1 == 0 ) then
    exit 2
endif

set UPTO=$1
set i=0

while( $i < $UPTO )
    "$me/locust.csh" >& /dev/null &
    set pids=( $pids $! )
    @ i++
end

echo waiting
wait

echo good: `cat good | wc -l`
echo bad: `cat bad | wc -l`
