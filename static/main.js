document.addEventListener('DOMContentLoaded', function () {
  const form = document.getElementById('note-form');
  const noteContent = document.getElementById('note-content');
  const notesList = document.getElementById('notes-list');

  async function fetchNotes() {
      const response = await fetch('/api/notes');
      const notes = await response.json();
      notesList.innerHTML = '';
      notes.forEach(note => {
          const li = document.createElement('li');
          li.textContent = note.content;
          notesList.appendChild(li);
      });
  }

  form.addEventListener('submit', async function (event) {
      event.preventDefault();
      const content = noteContent.value;
      if (content) {
          const response = await fetch('/api/notes', {
              method: 'POST',
              headers: {
                  'Content-Type': 'application/json'
              },
              body: JSON.stringify({ id: 1, content })
          });
          const newNote = await response.json();
          const li = document.createElement('li');
          li.textContent = newNote.content;
          notesList.appendChild(li);
          noteContent.value = '';
      }
  });

  fetchNotes();
});
