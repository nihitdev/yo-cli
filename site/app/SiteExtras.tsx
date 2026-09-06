"use client";

import { useEffect, useState } from "react";

export function SiteExtras() {
  const [position, setPosition] = useState({ x: -40, y: -40 });
  const [active, setActive] = useState(false);

  useEffect(() => {
    function move(event: PointerEvent) {
      setPosition({ x: event.clientX, y: event.clientY });
    }

    function over(event: PointerEvent) {
      const target = event.target as HTMLElement;
      setActive(Boolean(target.closest("a, button")));
    }

    window.addEventListener("pointermove", move);
    document.addEventListener("pointerover", over);
    return () => {
      window.removeEventListener("pointermove", move);
      document.removeEventListener("pointerover", over);
    };
  }, []);

  return (
    <>
      <span
        className={`custom-cursor ${active ? "is-active" : ""}`}
        style={{ transform: `translate3d(${position.x}px, ${position.y}px, 0)` }}
        aria-hidden="true"
      />
      <aside className="contact-dock" aria-label="Contact yoo">
        <span className="contact-label">Say hello</span>
        <a
          className="contact-action chat-action"
          href="https://github.com/nihitdev/yo-cli/discussions"
          aria-label="Chat in GitHub Discussions"
          title="Chat"
        >
          <span className="chat-icon" aria-hidden="true"><i /><i /><i /></span>
        </a>
      </aside>
    </>
  );
}
