/** Hafif yay (spring) çözücü. Yalnızca hareket sırasında rAF çalıştırır; durağanken sıfır CPU. */
export interface SpringOpts { stiffness?: number; damping?: number; mass?: number; speed?: number }

export function animateSpring(
  from: number, to: number, onUpdate: (v: number) => void, opts: SpringOpts = {},
): () => void {
  const { stiffness = 260, damping = 24, mass = 1, speed = 1 } = opts;
  let x = from, v = 0, last = performance.now(), raf = 0, stopped = false;
  const step = (now: number) => {
    if (stopped) return;
    const dt = Math.min((now - last) / 1000, 1 / 30) * speed;
    last = now;
    const a = (-stiffness * (x - to) - damping * v) / mass;
    v += a * dt; x += v * dt;
    onUpdate(x);
    if (Math.abs(v) < 0.01 && Math.abs(x - to) < 0.01) { onUpdate(to); return; }
    raf = requestAnimationFrame(step);
  };
  raf = requestAnimationFrame(step);
  return () => { stopped = true; cancelAnimationFrame(raf); };
}
