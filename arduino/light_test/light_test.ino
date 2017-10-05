static const int IN_BUF_SIZE = 14;
static const int NUM_LIGHTS = 2;

struct Colour{
  int r, g, b;
};

struct Pin{
  int r, g, b;
};

struct Light {
  Pin pin;
  Colour col;
};

enum Command { NONE, LIGHT };
enum C_Col { RED, GREEN, BLUE, NOT_COL};
enum Result { ERR, OK };

Light lights[NUM_LIGHTS];

void setup() {
  const Pin pins[NUM_LIGHTS] = { {11, 10, 9}, {6, 5, 3} };
  for(int i = 0; i < NUM_LIGHTS; ++i){
    lights[i].pin = pins[i];
    lights[i].col = {0, 0, 0};
  }
  Serial.begin(9600);
  for(int i = 0; i < NUM_LIGHTS; ++i){
    pinMode(lights[i].pin.r, OUTPUT);
    pinMode(lights[i].pin.g, OUTPUT);
    pinMode(lights[i].pin.b, OUTPUT);
  }
}

Result get_msg(char in_buf[]){
  byte size = Serial.readBytesUntil('x', in_buf, IN_BUF_SIZE);
  if((int)size > 0 && (int)size <= IN_BUF_SIZE){
    return OK;
  }else{
    return ERR;
  }
}

Result update_light(char in_buf[]){
  int index = 0;
  Result result = OK;
  if( isDigit(in_buf[0]) ){
    int l_num = in_buf[0] - '0';
    if(l_num < 0 || l_num > NUM_LIGHTS){
      result = ERR;
      Serial.println("Light out of range");
    }
    int col_vals[3] = {0, 0, 0};
    index = 1;
    for(int n = 0; n < 3; ++n){
      char val[4];
      for(int i = 0; i < 4; ++i){
        if( isDigit(in_buf[index + i]) ){
          val[i] = in_buf[index + i];
        }else{
          if(i == 0){
            result = ERR;
            Serial.println("number too short");
          }
          val[i] = '\0';
          index += i + 1;
          break;
        }
      }
      col_vals[n] = atoi(val);
    }
    if(result == OK){
      lights[l_num].col.r = col_vals[0];
      lights[l_num].col.g = col_vals[1];
      lights[l_num].col.b = col_vals[2];
    }
  }else{
    result = ERR;
    Serial.println("Not a digit or in light range");
  }
  return result;
}

void handle_input(){
  char in_buf[IN_BUF_SIZE];
  Command command = NONE;
  C_Col colour = NOT_COL;
  int index = 0;
  int col_vals[3] = {0, 0, 0};

  while (Serial.available() > 0) {
    char in = Serial.read();
    switch(in){
      case 'l':
        {
          command = LIGHT;
          if(get_msg(in_buf) == OK){
            if(update_light(in_buf) == ERR){
              Serial.println("Bad Message format");
            }
          }else{
            Serial.println("Error recieving message");
          }
        }
        break;
      default:
        break;
    }

  }
}

void loop() {
  handle_input();
  for(int i = 0; i < NUM_LIGHTS; ++i){
    analogWrite(lights[i].pin.r, lights[i].col.r);
    analogWrite(lights[i].pin.g, lights[i].col.g);
    analogWrite(lights[i].pin.b, lights[i].col.b);
  }
}
