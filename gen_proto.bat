cd ./thrift/windows
thrift -out ../../crates/proto/src --gen rs ../../proto/common.thrift
thrift -out ../../crates/proto/src --gen rs ../../proto/client.thrift
thrift -out ../../crates/proto/src --gen rs ../../proto/gate.thrift
thrift -out ../../crates/proto/src --gen rs ../../proto/hub.thrift
cd ../../
pause