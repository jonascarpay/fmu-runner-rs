typedef int Id;

typedef struct Vec2 {
  double x;
  double y;
} Vec2;

typedef struct Vec2 (*Handler)(double);

void get_force(Id instance_id, double t, struct Vec2 *force);

void register_handler(Id instance_id, Handler handler);
