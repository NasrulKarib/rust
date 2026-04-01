const target = document.getElementById('typewriter');

let index = 0;
let text = 'Hello world from Rust backend!';

function typeNextCharacter() {
    if (!target) {
        return;
    }

    if (index <= text.length) {
        target.textContent = text.slice(0, index);
        index += 1;
        setTimeout(typeNextCharacter, 100);
    }
}

async function loadText() {
    try {
        const response = await fetch('/api/hello');

        if (!response.ok) {
            throw new Error('Failed to load backend message.');
        }

        const data = await response.json();

        if (data && typeof data.message === 'string' && data.message.trim().length > 0) {
            text = data.message;
        }
    } catch (error) {
        // Keep default text when backend is unavailable.
    }

    typeNextCharacter();
}

document.addEventListener('DOMContentLoaded', loadText);