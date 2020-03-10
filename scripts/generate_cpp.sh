#!/usr/bin/env bash

SCRIPTS_DIR=$(dirname "$0")
source $SCRIPTS_DIR/common.sh

echo "generate cpp code..."

silkproto_ROOT="$SCRIPTS_DIR/.."
GOGO_ROOT=${silkproto_ROOT}/_vendor/src/github.com/gogo/protobuf
GRPC_INCLUDE=.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf:../include

cd $silkproto_ROOT
rm -rf proto-cpp && mkdir -p proto-cpp
rm -rf cpp/silkproto && mkdir cpp/silkproto

cp proto/* proto-cpp/

sed_inplace '/gogo.proto/d' proto-cpp/*
sed_inplace '/option\ *(gogoproto/d' proto-cpp/*
sed_inplace -e 's/\[.*gogoproto.*\]//g' proto-cpp/*

push proto-cpp
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/silkproto *.proto || exit $?
protoc -I${GRPC_INCLUDE} --grpc_out ../cpp/silkproto --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` *.proto || exit $?
pop

push include
protoc -I${GRPC_INCLUDE} --cpp_out ../cpp/silkproto *.proto || exit $?
pop

rm -rf proto-cpp