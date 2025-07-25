class ChatMessage extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  static get observedAttributes() {
    return ['author', 'content', 'avatar'];
  }
  
  connectedCallback() {
    this.render();
  }
  
  attributeChangedCallback() {
    this.render();
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          margin-bottom: 15px;
        }
        
        .message {
          display: flex;
          align-items: flex-start;
          gap: 12px;
        }
        
        .avatar {
          width: 40px;
          height: 40px;
          border-radius: 50%;
          background: linear-gradient(45deg, #667eea, #764ba2);
          color: white;
          display: flex;
          align-items: center;
          justify-content: center;
          font-weight: bold;
          flex-shrink: 0;
          overflow: hidden;
        }
        
        .avatar img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }
        
        .message-content {
          background: white;
          padding: 12px 16px;
          border-radius: 18px;
          box-shadow: 0 2px 8px rgba(0,0,0,0.1);
          max-width: 70%;
        }
        
        .author {
          font-weight: 600;
          color: #333;
          margin-bottom: 4px;
          font-size: 0.9em;
        }
        
        .content {
          color: #555;
          line-height: 1.4;
          word-wrap: break-word;
        }
        
        /* Animation d'apparition */
        :host {
          animation: slideIn 0.3s ease-out;
        }
        
        @keyframes slideIn {
          from {
            opacity: 0;
            transform: translateY(10px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }
      </style>
      <div class="message">
        <div class="avatar">
          <img src="${this.getAttribute('avatar')}" alt="avatar">
        </div>
        <div class="message-content">
          <div class="author">${this.getAttribute('author')}</div>
          <div class="content">${this.getAttribute('content')}</div>
        </div>
      </div>
    `;
  }
}

customElements.define('chat-message', ChatMessage);