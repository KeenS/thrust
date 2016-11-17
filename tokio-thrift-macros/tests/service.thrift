namespace rust thrift_service;


service Foo {
  bool bar(1: string token);
}