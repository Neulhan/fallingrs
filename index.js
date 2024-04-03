import init, * as FallingRS from "./dev/fallingrs.js";

export default class FallingJS {
  constructor({
    frequency = 1,
    minRadius = 1,
    maxRadius = 3,
    minSpeed = 1,
    maxSpeed = 3,
    minAngle = -0.1,
    maxAngle = 0.1,
    colors = ["#FFF"],
    type_ = "Square",
    text = "*",
    el = "body",
    wasm = false,
  } = {}) {
    this.init = init().then(() => {
      this.scene = new FallingRS.Scene(
        new FallingRS.FallingConfig(
          frequency,
          minRadius,
          maxRadius,
          minSpeed,
          maxSpeed,
          minAngle,
          maxAngle,
          colors,
          FallingRS.FlakeType[type_],
          text,
          el
        )
      );
    });
  }

  async start() {
    await this.init;
    this.scene.resize();
    window.addEventListener("resize", this.scene.resize.bind(this));

    function animate(t) {
      this.scene.render(t);
      requestAnimationFrame(animate.bind(this));
    }
    requestAnimationFrame(animate.bind(this));
  }
}
