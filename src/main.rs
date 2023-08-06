fn main() {
 let events = winit::event_loop::EventLoop::new();

 match winit::window::WindowBuilder::new().build(&events) {
  Ok(window) => {
   println!("window: {:?}", window);

   events.run(move |event, _, control_flow|{ 
    control_flow.set_poll();
    control_flow.set_wait();

    match event {
     winit::event::Event::WindowEvent { event, .. } => { 
      match event {
       winit::event::WindowEvent::KeyboardInput { input, .. } => { 
        println!("input: {:?}", input);

       }//winit::event::WindowEvent::KeyboardInput { input, .. } => { 

       winit::event::WindowEvent::CloseRequested => { window.set_visible(false); }

       _ => { }
      }//match event {
     }//winit::event::Event::WindowEvent { event, .. } => { 
     _ => { }
    }//match event {
   });//events.run(move |event, _, control_flow|{ 
  }//Ok(window) => {

  Err(error) => {
   println!("error: {:?}", error);

  }//Err(error) => {
 }//match winit::window::WindowBuilder::new().build(&events) {
}//fn main() {
