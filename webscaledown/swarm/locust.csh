#!/bin/csh
set HOST=localhost
set TMPFILE=`mktemp`
set CODE=`curl -v -L -s -o $TMPFILE -z $TMPFILE -w "%{http_code}" -F resolution=40x30 -F image=@../test.jpg -X POST ${HOST}:8084/downscale`
#set CODE=`curl -v -L -s -o $TMPFILE -z $TMPFILE -w "%{http_code}" -F resolution=40x30 -F image=@/dev/zero -X POST ${HOST}:8084/downscale`
file $TMPFILE
switch($CODE)
    case 200:
        set status=0
        echo $CODE >> good
        breaksw
    case 202:
        set status=1
        echo $CODE >> bad
        breaksw
    default:
        cat $TMPFILE
        set status=1
        echo $CODE >> bad
        breaksw
endsw
rm $TMPFILE
