cd ./thrift/windows
thrift -out ../../src --gen rs ../../proto/common.thrift
thrift -out ../../src --gen rs ../../proto/client.thrift
thrift -out ../../src --gen rs ../../proto/gate.thrift
thrift -out ../../src --gen rs ../../proto/hub.thrift
thrift -out ../../src --gen rs ../../proto/dbproxy.thrift
cd ../../
pause