function loader_screen_wasm() {
    const LOADER_ID = 'wasm-loader-js';
    if (document.getElementById(LOADER_ID)) return;

    const loader = document.createElement('div');
    loader.id = LOADER_ID;

    const spinner = document.createElement('div');
    spinner.className = 'wasm-loader-spinner';

    const text = document.createElement('p');
    text.className = 'wasm-loader-text';
    text.textContent = 'wAIves loading...';

    loader.appendChild(spinner);
    loader.appendChild(text);
    document.body.prepend(loader);

    // Inject CSS style
    const style = document.createElement('style');
    style.textContent = `
      #${LOADER_ID} {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background-color: #789fc2;   /* fond cohérent avec le thème */
        color: white;
        font-family: 'Arial', sans-serif;
        gap: 1rem;
        z-index: 10000;              /* au-dessus de tout */
      }

      #${LOADER_ID} .wasm-loader-spinner {
        width: 60px;
        height: 60px;
        border: 6px solid rgba(255, 255, 255, 0.3);
        border-top-color: #c3aa19;   /* couleur or du thème */
        border-radius: 50%;
        animation: wasm-spin 0.8s linear infinite;
      }

      @keyframes wasm-spin {
        to { transform: rotate(360deg); }
      }

      #${LOADER_ID} .wasm-loader-text {
        font-size: 1.2rem;
        opacity: 0.9;
      }
    `;
    document.head.appendChild(style);
};

document.addEventListener('DOMContentLoaded', function () { loader_screen_wasm(); });