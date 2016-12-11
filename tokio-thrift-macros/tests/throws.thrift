//namespace rust thrift_service

exception Ex1 {
}

exception Ex2 {
}


service Foo {
  bool bar(1: string token) throws (1: Ex1 ex1, 2: Ex2 ex2);
}