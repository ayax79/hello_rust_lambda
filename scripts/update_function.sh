#!/bin/sh

ZIP_FILE="./aws_lambda.zip"

if [[ ! -f ${ZIP_FILE} ]]
then
    (>&2 echo "No zip file names ${ZIP_FILE} could be found")
    exit 1
fi

aws lambda update-function-code \
--function-name hello-rust \
--zip-file fileb://${ZIP_FILE} \
--publish