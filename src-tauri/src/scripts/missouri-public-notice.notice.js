
function init() {
  const content = document.querySelector('.notice').innerText
  const pdf = Array.from(document.querySelector('.notice').querySelectorAll('a')).map(({ href }) => href).filter(link => link.includes('PDF'))

  console.log("Notice PDF Content", { content })
  console.log("Notice has PDF: ", pdf ? pdf[0] : null)

  if (pdf) {
    invoke("handle_missouri_public_notice", {
      content,
      pdf: pdf ? pdf[0] : null,
      link: window.location.href
    })
  } else {
    invoke("post_horton_dashboard_notice", {
      content,
      pdf: pdf ? pdf[0] : null,
      link: window.location.href
    })
  }

  // if (window.formInterval) {
  //   clearInterval(window.formInterval);
  // }
  // window.formInterval = setInterval(() => {
  //   const form = document.querySelector("form textarea");
  //   if (!form) return;
  //   clearInterval(window.formInterval);
  //   cmdTip();
  //   new MutationObserver(function (mutationsList) {
  //     for (const mutation of mutationsList) {
  //       if (mutation.target.getAttribute('id') === '__next') {
  //         initDom();
  //         cmdTip();
  //       }
  //       if (mutation.target.getAttribute('class') === 'chat-model-cmd-list') {
  //         // The `chatgpt prompt` fill can be done by clicking on the event.
  //         const searchDom = document.querySelector("form .chat-model-cmd-list>div");
  //         const searchInput = document.querySelector('form textarea');
  //         if (!searchDom) return;
  //         searchDom.addEventListener('click', (event) => {
  //           const item = event.target.closest("div");
  //           if (item) {
  //             const val = decodeURIComponent(item.getAttribute('data-prompt'));
  //             searchInput.value = val;
  //             document.querySelector('form textarea').focus();
  //             initDom();
  //           }
  //         });
  //       }
  //     }
  //   }).observe(document.body, {
  //     childList: true,
  //     subtree: true,
  //   });
  // }, 300);
}

if (
  document.readyState === "complete" ||
  document.readyState === "interactive"
) {
  init();
} else {
  document.addEventListener("DOMContentLoaded", init);
}