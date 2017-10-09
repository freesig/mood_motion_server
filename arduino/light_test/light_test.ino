static const int IN_BUF_SIZE = 14;
static const int NUM_LIGHTS = 2;

const uint8_t PROGMEM gamma8[] = {
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  1,  1,  1,  1,
    1,  1,  1,  1,  1,  1,  1,  1,  1,  2,  2,  2,  2,  2,  2,  2,
    2,  3,  3,  3,  3,  3,  3,  3,  4,  4,  4,  4,  4,  5,  5,  5,
    5,  6,  6,  6,  6,  7,  7,  7,  7,  8,  8,  8,  9,  9,  9, 10,
   10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14, 14, 15, 15, 16, 16,
   17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25,
   25, 26, 27, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36,
   37, 38, 39, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50,
   51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68,
   69, 70, 72, 73, 74, 75, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89,
   90, 92, 93, 95, 96, 98, 99,101,102,104,105,107,109,110,112,114,
  115,117,119,120,122,124,126,127,129,131,133,135,137,138,140,142,
  144,146,148,150,152,154,156,158,160,162,164,167,169,171,173,175,
  177,180,182,184,186,189,191,193,196,198,200,203,205,208,210,213,
  215,218,220,223,225,228,231,233,236,239,241,244,247,249,252,255 };

struct Colour {
  int r, g, b;
};

struct Pin {
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
  for (int i = 0; i < NUM_LIGHTS; ++i) {
    lights[i].pin = pins[i];
    lights[i].col = {0, 0, 0};
  }
  Serial.begin(57600);
  for (int i = 0; i < NUM_LIGHTS; ++i) {
    pinMode(lights[i].pin.r, OUTPUT);
    pinMode(lights[i].pin.g, OUTPUT);
    pinMode(lights[i].pin.b, OUTPUT);
  }
}

Result get_msg(char in_buf[]) {
  byte size = Serial.readBytesUntil('x', in_buf, IN_BUF_SIZE);
  if ((int)size > 0 && (int)size <= IN_BUF_SIZE) {
    return OK;
  } else {
    return ERR;
  }
}

Result update_light(char in_buf[]) {
  int index = 0;
  Result result = OK;
  if ( isDigit(in_buf[0]) ) {
    int l_num = in_buf[0] - '0';
    if (l_num < 0 || l_num > NUM_LIGHTS) {
      result = ERR;
      Serial.println("Light out of range");
    }
    int col_vals[3] = {0, 0, 0};
    index = 1;
    for (int n = 0; n < 3; ++n) {
      char val[4];
      for (int i = 0; i < 4; ++i) {
        if ( isDigit(in_buf[index + i]) ) {
          val[i] = in_buf[index + i];
        } else {
          if (i == 0) {
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
    if (result == OK) {
      lights[l_num].col.r = col_vals[0];
      lights[l_num].col.g = col_vals[1];
      lights[l_num].col.b = col_vals[2];
    }
  } else {
    result = ERR;
    Serial.println("Not a digit or in light range");
  }
  return result;
}

void print_lights() {
  for (int i = 0; i < NUM_LIGHTS; ++i) {
    Serial.println("light");
    Serial.print("r: ");
    Serial.print(lights[i].col.r);
    Serial.print(" g: ");
    Serial.print(lights[i].col.g);
    Serial.print(" b: ");
    Serial.println(lights[i].col.b);
  }
}

void handle_input() {
  char in_buf[IN_BUF_SIZE];
  Command command = NONE;
  C_Col colour = NOT_COL;
  int index = 0;
  int col_vals[3] = {0, 0, 0};

  while (Serial.available() > 0) {
    char in = Serial.read();
    switch (in) {
      case 'l':
        {
          command = LIGHT;
          if (get_msg(in_buf) == OK) {
            if (update_light(in_buf) == ERR) {
              Serial.println("Bad Message format");
            } else {
              //print_lights();
            }
          } else {
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
  for (int i = 0; i < NUM_LIGHTS; ++i) {
    analogWrite( lights[i].pin.r, pgm_read_byte(&gamma8[lights[i].col.r]) );
    analogWrite(lights[i].pin.g, pgm_read_byte(&gamma8[lights[i].col.g]) );
    analogWrite(lights[i].pin.b, pgm_read_byte(&gamma8[lights[i].col.b]) );
  }

}
