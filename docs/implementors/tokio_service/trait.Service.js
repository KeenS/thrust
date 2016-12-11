(function() {var implementors = {};
implementors["tokio_service"] = [];implementors["tokio_proto"] = ["impl&lt;R1,&nbsp;R2,&nbsp;B1,&nbsp;B2,&nbsp;E&gt; <a class='trait' href='tokio_service/trait.Service.html' title='tokio_service::Service'>Service</a> for <a class='struct' href='tokio_proto/client/struct.Client.html' title='tokio_proto::client::Client'>Client</a>&lt;R1,&nbsp;R2,&nbsp;B1,&nbsp;B2,&nbsp;E&gt; <span class='where'>where R1: 'static, R2: 'static, B1: <a class='trait' href='futures/stream/trait.Stream.html' title='futures::stream::Stream'>Stream</a>&lt;Error=E&gt; + 'static, B2: 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='tokio_proto/enum.Error.html' title='tokio_proto::Error'>Error</a>&lt;E&gt;&gt; + 'static</span>","impl&lt;R1,&nbsp;R2&gt; <a class='trait' href='tokio_service/trait.Service.html' title='tokio_service::Service'>Service</a> for <a class='struct' href='tokio_proto/easy/struct.EasyClient.html' title='tokio_proto::easy::EasyClient'>EasyClient</a>&lt;R1,&nbsp;R2&gt; <span class='where'>where R1: 'static, R2: 'static</span>",];implementors["tokio_thrift"] = ["impl&lt;R1,&nbsp;R2,&nbsp;B1,&nbsp;B2,&nbsp;E&gt; <a class='trait' href='tokio_service/trait.Service.html' title='tokio_service::Service'>Service</a> for <a class='struct' href='tokio_proto/client/struct.Client.html' title='tokio_proto::client::Client'>Client</a>&lt;R1,&nbsp;R2,&nbsp;B1,&nbsp;B2,&nbsp;E&gt; <span class='where'>where B1: <a class='trait' href='futures/stream/trait.Stream.html' title='futures::stream::Stream'>Stream</a>&lt;Error=E&gt; + 'static, B2: 'static, E: <a class='trait' href='https://doc.rust-lang.org/nightly/core/convert/trait.From.html' title='core::convert::From'>From</a>&lt;<a class='enum' href='tokio_proto/error/enum.Error.html' title='tokio_proto::error::Error'>Error</a>&lt;E&gt;&gt; + 'static, R1: 'static, R2: 'static</span>","impl&lt;R1,&nbsp;R2&gt; <a class='trait' href='tokio_service/trait.Service.html' title='tokio_service::Service'>Service</a> for <a class='struct' href='tokio_proto/easy/struct.EasyClient.html' title='tokio_proto::easy::EasyClient'>EasyClient</a>&lt;R1,&nbsp;R2&gt; <span class='where'>where R1: 'static, R2: 'static</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()