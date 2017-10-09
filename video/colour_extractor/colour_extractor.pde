import processing.video.*;
Movie myMovie;
PImage frame;
int n = 0;
JSONArray values;
Selection area;
boolean start;
PVector min = null;
PVector max = null;

void setup() {
  size(800, 800);
  background(0);
  selectInput("Select a video to process:", "video_selected");
  values = new JSONArray();
  area = null;
  start = false;
}

void draw() {
  if (area != null && start) {
    if ( myMovie != null && floor( myMovie.time() ) >= floor( myMovie.duration() ) ) {
      saveJSONArray(values, "data/output.json");
      println("Done");
      exit();
    }
    if (myMovie != null && myMovie.available()) {
      myMovie.speed(1);
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
      println("time: " + myMovie.time() + " of " + myMovie.duration());
    }
  } else if (myMovie != null && myMovie.available()) {
    myMovie.speed(10);
    myMovie.read();
    image(myMovie, 0, 0);
    if (mousePressed) {
      strokeWeight(2);
      stroke(255);
      line(min.x, min.y, mouseX, min.y);
      line(min.x, min.y, min.x, mouseY);
      line(mouseX, min.y, mouseX, mouseY);
      line(min.x, mouseY, mouseX, mouseY);
    }
  }
}

void video_selected(File selection) {
  if (selection == null) {
    println("Window was closed or the user hit cancel.");
  } else {
    println("User selected " + selection.getPath());
    myMovie = new Movie(this, selection.getPath());
    myMovie.loop();
  }
}


color extract_color() {
  long r = 0;
  long g = 0;
  long b = 0;
  int a_width = int(area.max.x - area.min.x);
  int a_height = int(area.max.y - area.min.y);
  long total = a_width * a_height;
  for (int y = int(area.min.y); y < area.max.y; ++y) {
    for (int x = int(area.min.x); x < area.max.x; ++x) {
      int i = x + (y * myMovie.width);
      r += int(red(myMovie.pixels[i]));
      g += int(green(myMovie.pixels[i]));
      b += int(blue(myMovie.pixels[i]));
    }
  }
  return color(r/total, g/total, b/total);
}

class Selection {
  public PVector min, max;
  public Selection(PVector min, PVector max) {
    this.min = min;
    this.max = max;
  }
  public void re_align() {
    PVector t_min = new PVector(0, 0);
    PVector t_max = new PVector(0, 0);
    t_min.x = min(min.x, max.x);
    t_min.y = min(min.y, max.y);
    t_max.x = max(max.x, min.x);
    t_max.y = max(min.y, max.y);
    min = t_min;
    max = t_max;
  }
}

void keyPressed() {
  if (key == 's' && area != null) {
    start = true;
    println("area min x: " + area.min.x + " y " + area.min.y + " max x " +
      area.max.x + " y " + area.max.y);
      myMovie.noLoop();
      myMovie.jump(0);
    myMovie.play();
  }
}

void mousePressed() {
  min = new PVector(mouseX, mouseY);
}

void mouseReleased() {
  area = new Selection( min, new PVector(mouseX, mouseY) );
  area.re_align();
}