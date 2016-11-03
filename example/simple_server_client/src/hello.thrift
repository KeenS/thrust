namespace rust thrift;

service Hello {
  string hello_name(1: string name);
  string hello();
}
