use arch::vga;

mod arch;

extern "rust-intrinsic" {
    pub fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *T, offset: int) -> *T;
}

struct Point {
    x: uint,
    y: uint
}

pub struct Terminal {
    max: Point,
    current: Point,
    vga: vga::VGA
}

impl Terminal {

  pub fn new(vga: vga::VGA) -> Terminal {
    let (x, y) = (vga.x_max(), vga.y_max());
    Terminal { vga: vga, max: Point {x: x, y: y}, current: Point {x: 0, y: 0} }
  }

  pub fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point { x : 0, y : (self.current.y + 1) };
    } else {
      self.current = match self.current {
	Point { x: x, y: y }  if x == self.max.x - 1 && y == self.max.y - 1  => Point {x: 0, y: 0},
	Point { y: y, ..} if y == self.max.y - 1 => Point { x: 0, y: y + 1 },
	Point { x: x, y: y } => Point { x: x + 1, y: y }
      };
      self.vga.put((self.current.x, self.current.y), c, vga::White, vga::Black);
    }
  }
  
  pub fn clear_screen(&mut self) {
    range(0, self.max.x, |i| {
	range(0, self.max.y, |j| {
	  self.vga.put((i, j), 0 as u8, vga::Black, vga::Black);
	});
    });
  }
  
  pub fn put_hex(&mut self, c: u8) {
    let (upper, lower) = itoc(c);
    self.put_char('0' as u8);
    self.put_char('x' as u8);
    self.put_char(upper);
    self.put_char(lower);
  }
  
  pub fn put_int(&mut self, w: u32) {
    let (a, b, c, d, e, f, g, h) = wtoc(w);
    self.put_char('0' as u8);
    self.put_char('x' as u8);
    self.put_char(a);
    self.put_char(b);
    self.put_char(c);
    self.put_char(d);
    self.put_char(e);
    self.put_char(f);
    self.put_char(g);
    self.put_char(h);
  }

    
  pub fn print(&mut self, s:  &'static str) {
    let (ptr, buflen): (*u8, u32) = unsafe {
      transmute(s)
    };
    let mut i = 0;
    while i < buflen {
      unsafe {
	self.put_char(*offset(ptr, i as int)); 
      }
      i += 1;
    }
  }
  
  pub fn println(&mut self, s:  &'static str) {
    self.print(s);
    self.put_char('\n' as u8);
    //for c in s {
    //  self.put_char(s);
    //}
  }
  
}

fn itoc(i: u8) -> (u8, u8) {
  let lower = hex(0xf & i);
  let upper = hex((0xf0 & i) >> 4);
  (upper, lower)
}

fn wtoc(i: u32) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
  let (a, b) = itoc((i & 0xff) as u8);
  let (c, d) = itoc(((i & 0xff00) >> 8) as u8);
  let (e, f) = itoc(((i & 0xff0000) >> 16) as u8);
  let (g, h) = itoc(((i & 0xff000000) >> 24) as u8);
  
  (g, h, e, f, c, d, a, b) // TODO(ryan): why is it big endian?
}

fn hex(i: u8) -> u8 {
  match i {
    0..9 => 0x30 + i,
    0xA..0xF => 0x41 + (i - 0xA),
    _ => 122 // 'z'
  }
}

fn range(lo: uint, hi: uint, it: |uint| -> ()) {
    let mut iter = lo;
    while iter < hi {
	it(iter);
	iter += 1;
    }
}
