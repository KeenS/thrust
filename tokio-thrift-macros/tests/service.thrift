namespace rust thrift_service


service Foo {
  bool bar(1: string token);
  void baz(1: string token, 2: i32 id);
}