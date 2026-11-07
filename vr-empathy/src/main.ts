// EIN SOF — WebXR Empathy Test
// Copyright © 2026 Tuomas Johannes Karjalainen (s. 1971-09-07 13:30 UTC / 15:30 Turku, 070971-119X)

import * as THREE from "three";
import { VRButton } from "three/examples/jsm/webxr/VRButton.js";

interface Scenario {
  id: string;
  video: string;
  question: string;
  category: "suffering" | "injustice" | "altruism" | "cruelty";
}

const SCENARIOS: Scenario[] = [
  { id: "s1", video: "/vr/child-crying-360.mp4",
    question: "Mitä tunnet?", category: "suffering" },
  { id: "s2", video: "/vr/bullying-360.mp4",
    question: "Mitä tekisit?", category: "injustice" },
  { id: "s3", video: "/vr/animal-360.mp4",
    question: "Mitä ajattelet?", category: "cruelty" },
];

interface BiometricReading {
  hrvDrop: number;
  skinConductance: number;
  pupilDilation: number;
  microExpression: number;
  indifference: number;
}

const WEIGHTS = {
  hrv: 0.30, skin: 0.25, pupil: 0.20, expr: 0.15, ind: -0.10,
};

function scoreReading(r: BiometricReading): number {
  const raw =
    r.hrvDrop * WEIGHTS.hrv +
    r.skinConductance * WEIGHTS.skin +
    r.pupilDilation * WEIGHTS.pupil +
    r.microExpression * WEIGHTS.expr +
    r.indifference * WEIGHTS.ind;
  return Math.max(0, Math.min(1, raw));
}

function classify(score: number): string {
  if (score < 0.2) return "REJECTED";
  if (score < 0.5) return "RESTRICTED";
  if (score < 0.8) return "ACCEPTED";
  return "NODE";
}

/**
 * Web Bluetooth -pohjainen sykemittari.
 * Tukee Heart Rate Service (0x180D) -profiilia.
 */
class HeartRateMonitor {
  private device: any = null;
  private characteristic: any = null;
  private samples: number[] = [];

  async connect(): Promise<boolean> {
    try {
      this.device = await (navigator as any).bluetooth.requestDevice({
        filters: [{ services: ["heart_rate"] }],
      });
      const server = await this.device.gatt.connect();
      const service = await server.getPrimaryService("heart_rate");
      this.characteristic = await service.getCharacteristic("heart_rate_measurement");
      await this.characteristic.startNotifications();
      this.characteristic.addEventListener("characteristicvaluechanged",
        (ev: any) => {
          const value = ev.target.value;
          const flags = value.getUint8(0);
          const hr = (flags & 0x1) ? value.getUint16(1, true) : value.getUint8(1);
          this.samples.push(hr);
        });
      return true;
    } catch (e) {
      console.warn("BLE ei käytettävissä:", e);
      return false;
    }
  }

  start() { this.samples = []; }

  stop(): number {
    if (this.samples.length === 0) return 0;
    const avg = this.samples.reduce((a, b) => a + b, 0) / this.samples.length;
    const variance = this.samples.reduce((a, b) => a + (b - avg) ** 2, 0)
                     / this.samples.length;
    // HRV-approksimaatio: varianssi normalisoituna
    return Math.min(1, Math.sqrt(variance) / 20);
  }

  get connected() { return this.characteristic !== null; }
}

class EmpathyApp {
  private renderer: THREE.WebGLRenderer;
  private scene = new THREE.Scene();
  private camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
  private videoEl = document.createElement("video");
  private videoTexture!: THREE.VideoTexture;
  private hr = new HeartRateMonitor();
  private scores: number[] = [];
  private index = 0;
  private hud: HTMLElement;
  private questionEl: HTMLElement;

  constructor(canvas: HTMLCanvasElement) {
    this.renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.xr.enabled = true;
    document.body.appendChild(VRButton.createButton(this.renderer));

    this.hud = document.getElementById("hud")!;
    this.questionEl = document.getElementById("question")!;

    this.camera.position.set(0, 1.6, 0); // silmäkorkeus
    this.scene.add(this.camera);

    window.addEventListener("resize", () => this.onResize());

    this.videoEl.loop = true;
    this.videoEl.muted = true;
    this.videoEl.playsInline = true;
    this.videoEl.crossOrigin = "anonymous";

    this.renderer.setAnimationLoop(() => this.renderer.render(this.scene, this.camera));
  }

  private onResize() {
    this.camera.aspect = window.innerWidth / window.innerHeight;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(window.innerWidth, window.innerHeight);
  }

  async run(): Promise<number> {
    await this.hr.connect();

    for (const s of SCENARIOS) {
      await this.playScenario(s);
    }

    const avg = this.scores.reduce((a, b) => a + b, 0) / this.scores.length;
    this.questionEl.textContent =
      `Tulos: ${(avg * 100).toFixed(1)}% → ${classify(avg)}`;
    return avg;
  }

  private async playScenario(s: Scenario) {
    // Lataa 360-video
    this.videoEl.src = s.video;
    await this.videoEl.play();

    this.videoTexture = new THREE.VideoTexture(this.videoEl);
    this.videoTexture.mapping = THREE.EquirectangularReflectionMapping;
    this.scene.background = this.videoTexture;

    this.questionEl.textContent = s.question;
    this.hud.textContent = `E-SOF VK · ${this.index + 1}/${SCENARIOS.length}`;
    this.index++;

    // Tallennus 20 s
    this.hr.start();
    await new Promise((r) => setTimeout(r, 20_000));

    const hrv = this.hr.stop();
    const reading: BiometricReading = {
      hrvDrop: hrv,
      skinConductance: 0.5 + Math.random() * 0.3, // korvaa EDA-anturilla
      pupilDilation: 0.5 + Math.random() * 0.3,   // korvaa eye trackerillä
      microExpression: 0.5 + Math.random() * 0.3, // korvaa EMG:llä
      indifference: hrv < 0.05 ? 0.9 : 0.1,
    };
    this.scores.push(scoreReading(reading));
    this.videoEl.pause();
  }
}

// --- Käynnistys ---
const canvas = document.getElementById("c") as HTMLCanvasElement;
const app = new EmpathyApp(canvas);
window.addEventListener("click", () => app.run(), { once: true });
