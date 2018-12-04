FROM ayax79/rust_aws_lambda_build:latest

ENV OUTPUT_DIR=/output \
    ARTIFACTS_DIR=/artifacts \
    EXPORT_DIR=/export

ADD $SRC/src src
ADD $SRC/Cargo.toml .

RUN mkdir ${OUTPUT_DIR}
RUN cargo build --target $BUILD_TARGET --release 
RUN find target/${BUILD_TARGET}/release -maxdepth 1 -type f -executable -exec cp '{}' ${OUTPUT_DIR} \; 
RUN mkdir -p ${ARTIFACTS_DIR} 
RUN cp -r ${OUTPUT_DIR}/* ${ARTIFACTS_DIR}

WORKDIR $ARTIFACTS_DIR

RUN yum -y install zip

RUN find . -maxdepth 1 -type f -executable -exec zip aws_lambda.zip '{}' \;

RUN ls -a $ARTIFACTS_DIR

RUN mkdir -p $EXPORT_DIR

#Snapshot the directory
VOLUME $EXPORT_DIR

CMD find $ARTIFACTS_DIR -type f -name "aws_lambda.zip" -exec cp '{}' $EXPORT_DIR \;