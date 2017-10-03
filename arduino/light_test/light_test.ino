static const int IN_BUF_SIZE = 12;

struct Colour{
  int r, g, b;
};

struct Light {
  const int PIN = 3;
  Colour col;
};

Light light;

void setup() {
  light.col = {0, 0, 0};
  Serial.begin(9600);
  pinMode(light.PIN, OUTPUT);
}

void handle_input(){
  char input[IN_BUF_SIZE];
  while (Serial.available() > 0) {
    const byte size = Serial.readBytesUntil('\n', input, IN_BUF_SIZE);
    if(size > 1){
      switch (input[0]) {
        case 'l':
          {
            char * val = strtok(&input[1], ",");
            if(val != NULL){
              light.col.r = constrain( atoi(val), 0, 255);
            }
            strtok(NULL, ",");
            if(val != NULL){
              light.col.g = constrain( atoi(val), 0, 255);
            }
            strtok(NULL, ",");
            if(val != NULL){
              light.col.b = constrain( atoi(val), 0, 255);
            }
          }
          break;
        default:
          break;
      }
    }

  }
}

void loop() {
  handle_input();
  analogWrite(light.PIN, light.col.r);
}
