// UI helper functions
export function showStatus(message: string, type: string): void {
    const statusDiv = document.getElementById('status')!;
    statusDiv.textContent = message;
    statusDiv.className = type;
    statusDiv.classList.remove('hidden');
}

export function loadExample(example: string): void {
    const input = document.getElementById('mmlInput') as HTMLTextAreaElement;
    input.value = example;
    input.dispatchEvent(new Event('input'));
}
