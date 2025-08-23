import './style.css'

// Terminal colors mapping (VGA colors)
const VGA_COLORS = [
  '#000000', // Black
  '#0000AA', // Blue  
  '#00AA00', // Green
  '#00AAAA', // Cyan
  '#AA0000', // Red
  '#AA00AA', // Magenta
  '#AA5500', // Brown
  '#AAAAAA', // Light Gray
  '#555555', // Dark Gray
  '#5555FF', // Light Blue
  '#55FF55', // Light Green
  '#55FFFF', // Light Cyan
  '#FF5555', // Light Red
  '#FF55FF', // Pink
  '#FFFF55', // Yellow
  '#FFFFFF'  // White
];

let vgaEmulator = null;

// Load the WebAssembly module
async function loadWasm() {
  try {
    // For now, we'll simulate the WASM loading since we need to build it first
    console.log('Loading EspressOS WASM module...');
    
    // Simulate VGA emulator
    vgaEmulator = {
      writeString: (text, fg, bg) => {
        const output = document.getElementById('terminal-output');
        if (output) {
          const span = document.createElement('span');
          span.textContent = text;
          span.style.color = VGA_COLORS[fg] || VGA_COLORS[15];
          span.style.backgroundColor = VGA_COLORS[bg] || VGA_COLORS[0];
          output.appendChild(span);
        }
      },
      clear: () => {
        const output = document.getElementById('terminal-output');
        if (output) {
          output.innerHTML = '';
        }
      },
      newLine: () => {
        const output = document.getElementById('terminal-output');
        if (output) {
          output.appendChild(document.createElement('br'));
        }
      }
    };
    
    return true;
  } catch (error) {
    console.error('Failed to load WASM module:', error);
    return false;
  }
}

function simulateBoot() {
  if (!vgaEmulator) return;
  
  vgaEmulator.clear();
  vgaEmulator.writeString('Hello World!', 14, 0); // Yellow on black
  vgaEmulator.newLine();
  vgaEmulator.writeString('Welcome to EspressOS!', 14, 0); // Yellow on black
  vgaEmulator.newLine();
  vgaEmulator.newLine();
  vgaEmulator.writeString('EspressOS WebAssembly Demo', 15, 0); // White on black
  vgaEmulator.newLine();
  vgaEmulator.writeString('==========================', 15, 0);
  vgaEmulator.newLine();
  vgaEmulator.writeString('This is a web version of the EspressOS kernel output.', 7, 0); // Light gray
  vgaEmulator.newLine();
  vgaEmulator.writeString('The actual kernel runs on bare metal x86_64 hardware.', 7, 0);
  vgaEmulator.newLine();
  vgaEmulator.newLine();
  vgaEmulator.writeString('Status: ', 15, 0);
  vgaEmulator.writeString('RUNNING', 10, 0); // Light green
}

function runDemo() {
  if (!vgaEmulator) return;
  
  vgaEmulator.clear();
  vgaEmulator.writeString('EspressOS Kernel Demo', 14, 0);
  vgaEmulator.newLine();
  vgaEmulator.writeString('====================', 14, 0);
  vgaEmulator.newLine();
  vgaEmulator.newLine();
  
  const demoSteps = [
    { text: '[OK] Initializing VGA buffer...', color: 10 },
    { text: '[OK] Setting up interrupt handlers...', color: 10 },
    { text: '[OK] Memory management initialized...', color: 10 },
    { text: '[OK] Boot process complete!', color: 10 },
    { text: '', color: 15 },
    { text: 'Available commands:', color: 15 },
    { text: '  help    - Show this help', color: 7 },
    { text: '  clear   - Clear screen', color: 7 },
    { text: '  reboot  - Restart system', color: 7 },
    { text: '  halt    - Stop system', color: 7 },
  ];
  
  let step = 0;
  const timer = setInterval(() => {
    if (step < demoSteps.length) {
      const { text, color } = demoSteps[step];
      vgaEmulator.writeString(text, color, 0);
      vgaEmulator.newLine();
      step++;
    } else {
      clearInterval(timer);
      vgaEmulator.newLine();
      vgaEmulator.writeString('espressos> ', 15, 0);
    }
  }, 500);
}

function clearScreen() {
  if (!vgaEmulator) return;
  vgaEmulator.clear();
  vgaEmulator.writeString('EspressOS Terminal Cleared', 7, 0);
  vgaEmulator.newLine();
  vgaEmulator.writeString('espressos> ', 15, 0);
}

// Initialize the application
async function init() {
  const wasmLoaded = await loadWasm();
  
  if (wasmLoaded) {
    // Set up event listeners
    document.getElementById('boot-btn')?.addEventListener('click', simulateBoot);
    document.getElementById('clear-btn')?.addEventListener('click', clearScreen);
    document.getElementById('demo-btn')?.addEventListener('click', runDemo);
    
    // Initial boot simulation
    setTimeout(simulateBoot, 1000);
  } else {
    document.getElementById('terminal-output').textContent = 'Failed to load EspressOS WASM module';
  }
}

// Start the application
init();
