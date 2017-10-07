import processing.video.*;
Movie myMovie;
PImage frame;
int n = 0;
JSONArray values;

void setup() {
  size(50, 50);
  background(0);
  selectInput("Select a video to process:", "video_selected");
  values = new JSONArray();
}

void draw() {

  if(myMovie != null && myMovie.time() >= myMovie.duration()){
    saveJSONArray(values, "data/output.json");
    println("Done");
    exit();
  }
  if (myMovie != null && myMovie.available()) {
    myMovie.speed(10);
    myMovie.read();
    myMovie.loadPixels();
    color c = extract_color();
    background(c);
    JSONObject colour = new JSONObject();
    colour.setInt("r", int( red(c) ) );
    colour.setInt("g", int( green(c) ) );
    colour.setInt("b", int( blue(c) ) );
    values.setJSONObject(n, colour);
    ++n;
    println("n: " + n);
    
  }
}

void video_selected(File selection) {
  if (selection == null) {
    println("Window was closed or the user hit cancel.");
  } else {
    println("User selected " + selection.getPath());
    myMovie = new Movie(this, selection.getPath());
    
    myMovie.play();
  }
}

color extract_color(){
  long r = 0;
  long g = 0;
  long b = 0;
  long total = myMovie.width + myMovie.height;
  for(int i = 0; i < total; ++i){
    r = int(red(myMovie.pixels[i]));
    g = int(green(myMovie.pixels[i]));
    b = int(blue(myMovie.pixels[i]));
  }
  return color(r/total, g/total, b/total);
}