#!/bin/sh

aws lambda invoke --function-name hello-rust \
--payload '{"email":"foo@bar.com","firstName":"foo","lastName":"bar"}' \
./test-output.json