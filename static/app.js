async function register() {
    const username = document.getElementById('username').value;
    const response = await fetch('/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username })
    });
    const user = await response.json();
    alert(`Registered as ${user.username} (ID: ${user.id})`);
}

async function sendMessage() {
    const sender_id = document.getElementById('sender_id').value;
    const recipient_id = document.getElementById('recipient_id').value;
    const content = document.getElementById('message').value;
    const expires_in = document.getElementById('expires_in').value;
    const response = await fetch('/message', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ sender_id, recipient_id, content, expires_in })
    });
    const message = await response.json();
    displayMessages();
}

async function uploadFile() {
    const fileInput = document.getElementById('file');
    const message_id = document.getElementById('file_message_id').value;
    const formData = new FormData();
    formData.append('file', fileInput.files[0]);
    await fetch(`/upload?message_id=${message_id}`, {
        method: 'POST',
        body: formData
    });
    alert('File uploaded');
}

async function displayMessages() {
    const user_id = document.getElementById('sender_id').value;
    const response = await fetch(`/messages/${user_id}`);
    const messages = await response.json();
    const messagesDiv = document.getElementById('messages');
    messagesDiv.innerHTML = messages.map(m => `<p>${m.content}</p>`).join('');
}

async function startCall() {
    const peerConnection = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
    });
    const localVideo = document.getElementById('localVideo');
    const remoteVideo = document.getElementById('remoteVideo');

    const stream = await navigator.mediaDevices.getUserMedia({ audio: true, video: true });
    localVideo.srcObject = stream;
    stream.getTracks().forEach(track => peerConnection.addTrack(track, stream));

    peerConnection.ontrack = (event) => {
        remoteVideo.srcObject = event.streams[0];
    };

    peerConnection.onicecandidate = async (event) => {
        if (event.candidate) {
            await fetch('/signal', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ sdp: '', candidate: JSON.stringify(event.candidate) })
            });
        }
    };

    const offer = await peerConnection.createOffer();
    await peerConnection.setLocalDescription(offer);
    await fetch('/signal', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ sdp: JSON.stringify(offer), candidate: null })
    });

    // Simplified: In a real app, handle incoming SDP/candidates via WebSocket
    alert('Call initiated (connect to recipient manually)');
}
