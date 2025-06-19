use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// tokio - async runtime
// building blocks for network apps
// - multi-threaded runtime
// - async std library
// - no blocking API
//  deisgned for
// - IO bound applications where each task spends most time waiting for IO
// - handling multiple web requests at once

// For async rust with a blocking API, use reqwest
// For reading lots of files, use a threadpool
// For multiple computations in parallel, use rayon

// async/await in Rust is used for writing async functions
// that look like sync code by transforming code block into
// a state machine implementing a trait called Future
// value returned by async fn is a Future.
// blocked Future will yield control of the thread, allowing Future to run on an executor

// await can be used to wait for the completion of another type
// that implements the Future trait. await doesn't block the current thread
// instead asynchronously waits for futures to complete
// allowing other tasks to run if the future is currrently unable to make progress
async fn greet(req: HttpRequest) -> impl Responder {
    // From an incoming request
    // get a reference to URL parameters container
    // URL parameter is a way to pass information about a click through its URL
    // here we het the matched parameter by name without type conversion or set to default
    // this is returned as an Option type which willl return contained Some value or provided default
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)

    // Responder is a trait implemented by types
    // and can be converted to an HTTP response
    // Any type implementing this trait can be used 
    // as return type of a handler.
    // Typically, handlers have only one return type
    // so we can use an opaque return type `-> impl Responder`
    // here format!() creates a String 
}

// async fn to enter async context
// executed by a runtime containing async task scheduler,
// and providing evented I/O, timers
// the macro #[tokio::main] transforms
// the async fn main() to a synchronous fn main()
// that initializes a runtime instance and executes
// the async main function
// we need main to be async since HttpServer::run is async
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // intanstiate and configure servers

    // create new HTTP server with application factory and bind to socket address
    // localhost (127.0.0.1) with port 8000 which is commonly used for web servers
    // and HTTP based applications, then start listening for incoming connections using run()
    // App factory provieds application builder for Actix-Web Application

    // HTTP Server handles all transport level concerns such as,
    // - where should application be listening for incoming requests? (bind to a TCP socket or a Unix domain socket)
    // - what is the max number of concurrent allowed connections? how many new connections per unit time?
    // - should TLS (transport layer security) be enabled?
    // Once HTTP server establishes connections with clients of API, Application handles the client requests

    // run() starts a number of HTTP workers in separate threads,
    // starting separate thread for each address and does accept() in a loop
    // requires a socket address to be bound and Actix system to be configured
    // accept() accepts new incoming connections 

    // web::get() creates a new royte with GET method guard
    // web::get() is shortcut for Route::new().guard(guard::Get())
    // Route allows add new endpoints to our application 
    // it combines a handler with a set of guards that specify 
    // conditions that a request must satisfy to be passed to handler
    // here the request is passed to handler iff its HTTP method is GET
    // GET is used to retrieve information from given server using a given URL
    // Route::to<F, args> sets handler function
    // Handler functions or Request handler functions handle request and send response back to client
    
    HttpServer::new(|| {
        App::new().route( "/",  web::get().to( greet))
        .route( "/{name}",  web::get().to( greet))
    }).bind("127.0.0.1:8000")?
    .run()
    .await
}