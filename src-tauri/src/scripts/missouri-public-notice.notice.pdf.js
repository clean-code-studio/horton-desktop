
function init() {
  const script = document.createElement('script');
  script.src = 'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/2.8.335/pdf.min.js';
  document.head.appendChild(script);
  (new Promise(resolve => script.addEventListener('load', resolve))).then(async () => {
    const loadingTask = pdfjsLib.getDocument(window.location.href);
    const pdf = await loadingTask.promise;
    const maxPages = pdf.numPages;
    
    const isValidPdf = async () => {
      if (maxPages === 1) {
        return true;
      } else if (maxPages === 2) {
        const secondPage = await pdf.getPage(2);
        const secondPageContent = await secondPage.getTextContent();
        return secondPageContent.items.length === 0;
      }
      return false;
    };

    if (await isValidPdf()) {
      const promises = [];
      for (let i = 1; i <= maxPages; i++) {
        const pagePromise = pdf.getPage(i);
        promises.push(pagePromise);
      }
      const pages = await Promise.all(promises);
      const textPromises = pages.map(page => page.getTextContent());
      const textContents = await Promise.all(textPromises);
      const notice = textContents.map(content => content.items.map(item => item.str).join(' ')).join('\\n');
      invoke('post_horton_dashboard_notice', { content: notice, link: window.location.href, pdf: window.location.href })
      return notice;
    } else {
      console.log("PDF doesn't meet the requirements.");

      return false; // or any default value you want to return when the PDF doesn't meet the requirements
    }

  })
}


if (
  document.readyState === "complete" ||
  document.readyState === "interactive"
) {
  init();
} else {
  document.addEventListener("DOMContentLoaded", init);
}