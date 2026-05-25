import { useEffect } from 'react';

/**
 * Custom hook to safely register global keyboard shortcuts
 * @param {Object} keyMap - An object mapping keys (e.g., 'F2', 's') to callback functions
 */
export function useHotkeys(keyMap) {
  useEffect(() => {
    const handleKeyDown = (event) => {
      // Formulate a normalized key identifier string
      const pressedKey = event.key;

      // Check if the pressed key exists in our custom mapping object
      if (keyMap[pressedKey]) {
        // Prevent default browser behaviors for system overrides (like F2 or F3)
        event.preventDefault();
        
        // Execute the associated action function
        keyMap[pressedKey](event);
      }
    };

    // Attach the listener globally to the window object
    window.addEventListener('keydown', handleKeyDown);

    // CLEANUP: Automatically unregister the listener to prevent memory leaks
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [keyMap]); // Re-binds cleanly if our callback actions shift
}