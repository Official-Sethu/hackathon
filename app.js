/**
 * VeritasGonka — Core Application Orchestrator
 * Connects UI interactions, animations, accessibility tools, and public ledger.
 */

document.addEventListener('DOMContentLoaded', () => {
  // DOM Elements
  const tabTextClaim = document.getElementById('tabTextClaim');
  const tabUrlClaim = document.getElementById('tabUrlClaim');
  const tabVideoClaim = document.getElementById('tabVideoClaim');
  const panelText = document.getElementById('panelText');
  const panelUrl = document.getElementById('panelUrl');
  const panelVideo = document.getElementById('panelVideo');
  const claimInput = document.getElementById('claimInput');
  const urlInput = document.getElementById('urlInput');
  const videoUrlInput = document.getElementById('videoUrlInput');
  const charCounter = document.getElementById('charCounter');
  const clearInputBtn = document.getElementById('clearInputBtn');
  const verifyBtn = document.getElementById('verifyBtn');
  const btnSpinner = document.getElementById('btnSpinner');
  const btnIcon = document.getElementById('btnIcon');
  const btnText = document.getElementById('btnText');
  const streamSection = document.getElementById('streamSection');
  const streamTimer = document.getElementById('streamTimer');
  const resultsSection = document.getElementById('resultsSection');
  const toastContainer = document.getElementById('toastContainer');
  const languageSelect = document.getElementById('languageSelect');
  const videoSpokenQuoteBox = document.getElementById('videoSpokenQuoteBox');
  const videoPlatformBadge = document.getElementById('videoPlatformBadge');
  const videoSpokenText = document.getElementById('videoSpokenText');

  // Stream Elements
  const terminalLog = document.getElementById('terminalLog');
  const statusChipDeepseek = document.getElementById('statusChipDeepseek');
  const statusChipMinimax = document.getElementById('statusChipMinimax');
  const statusChipKimi = document.getElementById('statusChipKimi');
  const progressFillDeepseek = document.getElementById('progressFillDeepseek');
  const progressFillMinimax = document.getElementById('progressFillMinimax');
  const progressFillKimi = document.getElementById('progressFillKimi');
  const reqIdDeepseek = document.getElementById('reqIdDeepseek');
  const reqIdMinimax = document.getElementById('reqIdMinimax');
  const reqIdKimi = document.getElementById('reqIdKimi');

  // Results Elements
  const truthScoreValue = document.getElementById('truthScoreValue');
  const gaugeProgressCircle = document.getElementById('gaugeProgressCircle');
  const verdictBadgePill = document.getElementById('verdictBadgePill');
  const verdictBadgeText = document.getElementById('verdictBadgeText');
  const verifiedClaimText = document.getElementById('verifiedClaimText');
  const verdictHeadline = document.getElementById('verdictHeadline');
  const verdictSummary = document.getElementById('verdictSummary');
  const fillFactuality = document.getElementById('fillFactuality');
  const valFactuality = document.getElementById('valFactuality');
  const fillConsensus = document.getElementById('fillConsensus');
  const valConsensus = document.getElementById('valConsensus');
  const fillFallacy = document.getElementById('fillFallacy');
  const valFallacy = document.getElementById('valFallacy');

  // Model Cards
  const stanceDeepseek = document.getElementById('stanceDeepseek');
  const reqValueDeepseek = document.getElementById('reqValueDeepseek');
  const latencyDeepseek = document.getElementById('latencyDeepseek');
  const tokensDeepseek = document.getElementById('tokensDeepseek');
  const assessmentDeepseek = document.getElementById('assessmentDeepseek');
  const confBarDeepseek = document.getElementById('confBarDeepseek');
  const confValDeepseek = document.getElementById('confValDeepseek');

  const stanceMinimax = document.getElementById('stanceMinimax');
  const reqValueMinimax = document.getElementById('reqValueMinimax');
  const latencyMinimax = document.getElementById('latencyMinimax');
  const tokensMinimax = document.getElementById('tokensMinimax');
  const assessmentMinimax = document.getElementById('assessmentMinimax');
  const confBarMinimax = document.getElementById('confBarMinimax');
  const confValMinimax = document.getElementById('confValMinimax');

  const stanceKimi = document.getElementById('stanceKimi');
  const reqValueKimi = document.getElementById('reqValueKimi');
  const latencyKimi = document.getElementById('latencyKimi');
  const tokensKimi = document.getElementById('tokensKimi');
  const assessmentKimi = document.getElementById('assessmentKimi');
  const confBarKimi = document.getElementById('confBarKimi');
  const confValKimi = document.getElementById('confValKimi');

  // Timeline & Fallacies
  const timelineSteps = document.getElementById('timelineSteps');
  const fallaciesList = document.getElementById('fallaciesList');
  const citationsList = document.getElementById('citationsList');

  // Utilities
  const ttsSpeakBtn = document.getElementById('ttsSpeakBtn');
  const ttsBtnText = document.getElementById('ttsBtnText');
  const generateCertBtn = document.getElementById('generateCertBtn');
  const copyShareLinkBtn = document.getElementById('copyShareLinkBtn');

  // Modals
  const factCardModal = document.getElementById('factCardModal');
  const closeFactCardModal = document.getElementById('closeFactCardModal');
  const certScoreNumber = document.getElementById('certScoreNumber');
  const certVerdictTitle = document.getElementById('certVerdictTitle');
  const certClaimText = document.getElementById('certClaimText');
  const certReq1 = document.getElementById('certReq1');
  const certReq2 = document.getElementById('certReq2');
  const certReq3 = document.getElementById('certReq3');
  const certDate = document.getElementById('certDate');
  const copyFactCardTextBtn = document.getElementById('copyFactCardTextBtn');
  const printFactCardBtn = document.getElementById('printFactCardBtn');

  const settingsModal = document.getElementById('settingsModal');
  const openSettingsBtn = document.getElementById('openSettingsBtn');
  const closeSettingsModal = document.getElementById('closeSettingsModal');
  const settingBaseUrl = document.getElementById('settingBaseUrl');
  const settingApiKey = document.getElementById('settingApiKey');
  const toggleApiKeyVisibility = document.getElementById('toggleApiKeyVisibility');
  const settingModel1 = document.getElementById('settingModel1');
  const settingModel2 = document.getElementById('settingModel2');
  const settingModel3 = document.getElementById('settingModel3');
  const saveSettingsBtn = document.getElementById('saveSettingsBtn');
  const resetSettingsBtn = document.getElementById('resetSettingsBtn');
  const testGonkaConnBtn = document.getElementById('testGonkaConnBtn');
  const diagResult = document.getElementById('diagResult');
  const footerConfigLink = document.getElementById('footerConfigLink');

  // Ledger Table
  const ledgerTableBody = document.getElementById('ledgerTableBody');
  const exportAuditLedgerBtn = document.getElementById('exportAuditLedgerBtn');

  // Internal State
  let currentActiveTab = 'text';
  let activeVerificationResult = null;
  let isSpeaking = false;
  let speechSynthUtterance = null;
  let timerInterval = null;

  // Clear stale ledger data from previous hardcoded build
  const LEDGER_VERSION = 'v2-live-ai';
  if (localStorage.getItem('veritas_ledger_version') !== LEDGER_VERSION) {
    localStorage.removeItem('veritas_ledger');
    localStorage.setItem('veritas_ledger_version', LEDGER_VERSION);
  }

  // Initial setup
  initSettingsValues();
  initLedger();


  // -------------------------------------------------------------
  // TAB NAVIGATION
  // -------------------------------------------------------------
  tabTextClaim.addEventListener('click', () => {
    currentActiveTab = 'text';
    tabTextClaim.classList.add('active');
    tabUrlClaim.classList.remove('active');
    if (tabVideoClaim) tabVideoClaim.classList.remove('active');
    panelText.classList.add('active');
    panelUrl.classList.remove('active');
    if (panelVideo) panelVideo.classList.remove('active');
  });

  tabUrlClaim.addEventListener('click', () => {
    currentActiveTab = 'url';
    tabUrlClaim.classList.add('active');
    tabTextClaim.classList.remove('active');
    if (tabVideoClaim) tabVideoClaim.classList.remove('active');
    panelUrl.classList.add('active');
    panelText.classList.remove('active');
    if (panelVideo) panelVideo.classList.remove('active');
  });

  if (tabVideoClaim) {
    tabVideoClaim.addEventListener('click', () => {
      currentActiveTab = 'video';
      tabVideoClaim.classList.add('active');
      tabTextClaim.classList.remove('active');
      tabUrlClaim.classList.remove('active');
      if (panelVideo) panelVideo.classList.add('active');
      panelText.classList.remove('active');
      panelUrl.classList.remove('active');
    });
  }

  // Character counter
  claimInput.addEventListener('input', () => {
    const len = claimInput.value.length;
    charCounter.textContent = `${len} / 2000 characters`;
  });

  // Clear button
  clearInputBtn.addEventListener('click', () => {
    claimInput.value = '';
    charCounter.textContent = '0 / 2000 characters';
    claimInput.focus();
  });

  // -------------------------------------------------------------
  // PRESET CLICKS
  // -------------------------------------------------------------
  document.querySelectorAll('.preset-tag').forEach(btn => {
    btn.addEventListener('click', () => {
      const sampleKey = btn.getAttribute('data-sample');
      const sampleData = SAMPLE_CLAIMS[sampleKey];
      if (sampleData) {
        if (sampleData.isVideo && tabVideoClaim) {
          tabVideoClaim.click();
          videoUrlInput.value = sampleData.videoUrl || sampleData.claim;
          triggerVerification(sampleData.videoUrl || sampleData.claim);
        } else {
          tabTextClaim.click();
          claimInput.value = sampleData.claim;
          charCounter.textContent = `${sampleData.claim.length} / 2000 characters`;
          triggerVerification(sampleData.claim);
        }
      }
    });
  });

  // -------------------------------------------------------------
  // VERIFY ACTION
  // -------------------------------------------------------------
  verifyBtn.addEventListener('click', () => {
    let claimToVerify = '';
    if (currentActiveTab === 'text') {
      claimToVerify = claimInput.value.trim();
      if (!claimToVerify) {
        showToast('Please enter a statement, headline, or claim to verify.');
        claimInput.focus();
        return;
      }
    } else if (currentActiveTab === 'url') {
      claimToVerify = urlInput.value.trim();
      if (!claimToVerify) {
        showToast('Please enter a valid news URL to examine.');
        urlInput.focus();
        return;
      }
    } else if (currentActiveTab === 'video') {
      claimToVerify = videoUrlInput.value.trim();
      if (!claimToVerify) {
        showToast('Please enter a TikTok, YouTube Short, or Instagram Reel link to fact-check.');
        videoUrlInput.focus();
        return;
      }
    }

    triggerVerification(claimToVerify);
  });

  let lastVerificationTime = 0;
  const CLIENT_RATE_LIMIT_COOLDOWN_MS = 5000;

  async function triggerVerification(text) {
    const now = Date.now();
    const elapsedSinceLast = now - lastVerificationTime;
    if (elapsedSinceLast < CLIENT_RATE_LIMIT_COOLDOWN_MS) {
      const remainingSec = Math.ceil((CLIENT_RATE_LIMIT_COOLDOWN_MS - elapsedSinceLast) / 1000);
      showToast(`Rate Limiting: Please wait ${remainingSec}s before running another check.`);
      return;
    }

    lastVerificationTime = Date.now();

    // UI Loading state
    verifyBtn.disabled = true;
    btnSpinner.style.display = 'inline-block';
    btnIcon.style.display = 'none';
    btnText.textContent = 'Routing through Gonka...';
    resultsSection.style.display = 'none';
    streamSection.style.display = 'block';

    // Reset stream state
    resetStreamPipeline();

    // Start timer
    const startTimestamp = Date.now();
    timerInterval = setInterval(() => {
      const sec = ((Date.now() - startTimestamp) / 1000).toFixed(1);
      streamTimer.textContent = `Elapsed: ${sec}s`;
    }, 100);

    appendLog('INIT', `Dispatching claim to Gonka Router at ${window.gonkaClient.config.baseUrl}`);

    try {
      const result = await window.gonkaClient.verifyClaim(text, (update) => {
        handleProgressUpdate(update);
      });

      clearInterval(timerInterval);
      activeVerificationResult = result;

      // Finish all pipeline bars
      progressFillDeepseek.style.width = '100%';
      progressFillMinimax.style.width = '100%';
      progressFillKimi.style.width = '100%';

      appendLog('SUCCESS', 'All 3 models reported conclusions. Consensus synthesized.');

      // Render Results
      setTimeout(() => {
        renderResults(result);
        addLedgerEntry(result);
        resultsSection.style.display = 'block';
        resultsSection.scrollIntoView({ behavior: 'smooth' });
      }, 400);

    } catch (err) {
      clearInterval(timerInterval);
      appendLog('ERROR', `Verification failed: ${err.message || 'AI Network Unreachable'}`);
      if (err.message && err.message.includes('429')) {
        showToast('Rate limit exceeded. Opening error page...');
        setTimeout(() => { window.location.href = '429.html'; }, 300);
      } else {
        showToast('AI Network unreachable. Redirecting to error page...');
        setTimeout(() => { window.location.href = '500.html'; }, 300);
      }
    } finally {
      btnSpinner.style.display = 'none';
      btnIcon.style.display = 'inline-block';

      // 4-second cooldown countdown on button to prevent spamming
      let cooldown = 4;
      verifyBtn.disabled = true;
      btnText.textContent = `Cooldown (${cooldown}s)...`;

      const cdInterval = setInterval(() => {
        cooldown--;
        if (cooldown > 0) {
          btnText.textContent = `Cooldown (${cooldown}s)...`;
        } else {
          clearInterval(cdInterval);
          verifyBtn.disabled = false;
          btnText.textContent = 'Verify via Gonka Router';
        }
      }, 1000);
    }
  }

  function resetStreamPipeline() {
    progressFillDeepseek.style.width = '10%';
    progressFillMinimax.style.width = '10%';
    progressFillKimi.style.width = '10%';

    statusChipDeepseek.textContent = 'Connecting...';
    statusChipDeepseek.className = 'status-tag pending';
    statusChipMinimax.textContent = 'Connecting...';
    statusChipMinimax.className = 'status-tag pending';
    statusChipKimi.textContent = 'Connecting...';
    statusChipKimi.className = 'status-tag pending';

    reqIdDeepseek.textContent = 'awaiting_dispatch...';
    reqIdMinimax.textContent = 'awaiting_dispatch...';
    reqIdKimi.textContent = 'awaiting_dispatch...';

    terminalLog.innerHTML = `
      <div class="feed-item">
        <span class="feed-time">[00:00.0]</span>
        <span class="feed-badge">INIT</span>
        <span class="feed-msg">Constructing OpenAI-compatible prompt payload for Gonka Router gateway...</span>
      </div>
    `;
  }

  function handleProgressUpdate(update) {
    if (update.stage === 'dispatching') {
      appendLog('DISPATCH', update.message);
    } else if (update.stage === 'model_start') {
      if (update.model === 'deepseek') {
        statusChipDeepseek.textContent = 'Evaluating...';
        progressFillDeepseek.style.width = '45%';
        appendLog('DISPATCH', `DeepSeek V4 connection established via Gonka Router`);
      } else if (update.model === 'minimax') {
        statusChipMinimax.textContent = 'Evaluating...';
        progressFillMinimax.style.width = '45%';
        appendLog('DISPATCH', `MiniMax M2.7 connection established via Gonka Router`);
      } else if (update.model === 'kimi') {
        statusChipKimi.textContent = 'Evaluating...';
        progressFillKimi.style.width = '45%';
        appendLog('DISPATCH', `Kimi K2.6 connection established via Gonka Router`);
      }
    } else if (update.stage === 'model_done') {
      if (update.model === 'deepseek') {
        statusChipDeepseek.textContent = 'Completed';
        statusChipDeepseek.className = 'status-tag success';
        progressFillDeepseek.style.width = '90%';
        reqIdDeepseek.textContent = update.reqId;
        appendLog('RESOLVED', `DeepSeek V4 finished in ${update.latency}ms [ID: ${update.reqId}]`);
      } else if (update.model === 'minimax') {
        statusChipMinimax.textContent = 'Completed';
        statusChipMinimax.className = 'status-tag success';
        progressFillMinimax.style.width = '90%';
        reqIdMinimax.textContent = update.reqId;
        appendLog('RESOLVED', `MiniMax M2.7 finished in ${update.latency}ms [ID: ${update.reqId}]`);
      } else if (update.model === 'kimi') {
        statusChipKimi.textContent = 'Completed';
        statusChipKimi.className = 'status-tag success';
        progressFillKimi.style.width = '90%';
        reqIdKimi.textContent = update.reqId;
        appendLog('RESOLVED', `Kimi K2.6 finished in ${update.latency}ms [ID: ${update.reqId}]`);
      }
    } else if (update.stage === 'synthesizing') {
      appendLog('SYNTHESIS', update.message);
    }
  }

  function appendLog(badge, msg) {
    const sec = (Date.now() % 60000 / 1000).toFixed(1);
    const item = document.createElement('div');
    item.className = 'feed-item';
    item.innerHTML = `
      <span class="feed-time">[${sec}s]</span>
      <span class="feed-badge">${badge}</span>
      <span class="feed-msg">${msg}</span>
    `;
    terminalLog.appendChild(item);
    terminalLog.scrollTop = terminalLog.scrollHeight;
  }

  // -------------------------------------------------------------
  // RENDER RESULTS
  // -------------------------------------------------------------
  function renderResults(data) {
    // Animate Truth Score & Circular SVG Gauge
    animateGauge(data.truthScore);

    // Headline & Summary
    verifiedClaimText.textContent = `"${data.claim}"`;
    verdictHeadline.textContent = data.headline;
    verdictSummary.textContent = data.summary;

    // Video Spoken Quote Box
    if (videoSpokenQuoteBox) {
      if (data.spokenTranscript || data.isVideo) {
        const platform = data.videoPlatform || (window.gonkaClient && window.gonkaClient.detectVideoPlatform ? window.gonkaClient.detectVideoPlatform(data.claim) : 'Short Video');
        const iconClass = platform.includes('TikTok') ? 'fa-brands fa-tiktok' :
                          platform.includes('YouTube') ? 'fa-brands fa-youtube' :
                          platform.includes('Instagram') ? 'fa-brands fa-instagram' : 'fa-solid fa-play';
        videoPlatformBadge.innerHTML = `<i class="${iconClass}"></i> ${platform}`;
        videoSpokenText.textContent = `"${data.spokenTranscript || data.claim}"`;
        videoSpokenQuoteBox.style.display = 'block';
      } else {
        videoSpokenQuoteBox.style.display = 'none';
      }
    }

    // Status Pill
    verdictBadgeText.textContent = data.verdictLabel;
    verdictBadgePill.className = `verdict-status-badge ${data.statusClass}`;

    // Sub-metrics
    valFactuality.textContent = `${data.factualityScore}%`;
    fillFactuality.style.width = `${data.factualityScore}%`;

    valConsensus.textContent = `${data.consensusScore}%`;
    fillConsensus.style.width = `${data.consensusScore}%`;

    valFallacy.textContent = data.fallacyRisk;
    fillFallacy.style.width = data.fallacyRisk === 'Critical' ? '90%' :
                              data.fallacyRisk === 'High' ? '75%' :
                              data.fallacyRisk === 'Moderate' ? '50%' : '15%';

    // 1. DeepSeek Card
    stanceDeepseek.textContent = data.deepseek.stance;
    stanceDeepseek.className = `stance-pill ${data.deepseek.stanceClass}`;
    reqValueDeepseek.textContent = data.deepseek.reqId;
    latencyDeepseek.innerHTML = `<i class="fa-regular fa-clock"></i> ${data.deepseek.latency} ms`;
    tokensDeepseek.innerHTML = `<i class="fa-solid fa-layer-group"></i> ${data.deepseek.tokens} tokens`;
    assessmentDeepseek.textContent = data.deepseek.assessment;
    confBarDeepseek.style.width = `${data.deepseek.confidence}%`;
    confValDeepseek.textContent = `${data.deepseek.confidence}%`;

    // 2. MiniMax Card
    stanceMinimax.textContent = data.minimax.stance;
    stanceMinimax.className = `stance-pill ${data.minimax.stanceClass}`;
    reqValueMinimax.textContent = data.minimax.reqId;
    latencyMinimax.innerHTML = `<i class="fa-regular fa-clock"></i> ${data.minimax.latency} ms`;
    tokensMinimax.innerHTML = `<i class="fa-solid fa-layer-group"></i> ${data.minimax.tokens} tokens`;
    assessmentMinimax.textContent = data.minimax.assessment;
    confBarMinimax.style.width = `${data.minimax.confidence}%`;
    confValMinimax.textContent = `${data.minimax.confidence}%`;

    // 3. Kimi Card
    stanceKimi.textContent = data.kimi.stance;
    stanceKimi.className = `stance-pill ${data.kimi.stanceClass}`;
    reqValueKimi.textContent = data.kimi.reqId;
    latencyKimi.innerHTML = `<i class="fa-regular fa-clock"></i> ${data.kimi.latency} ms`;
    tokensKimi.innerHTML = `<i class="fa-solid fa-layer-group"></i> ${data.kimi.tokens} tokens`;
    assessmentKimi.textContent = data.kimi.assessment;
    confBarKimi.style.width = `${data.kimi.confidence}%`;
    confValKimi.textContent = `${data.kimi.confidence}%`;

    // Reasoning Timeline
    timelineSteps.innerHTML = '';
    data.reasoningTrace.forEach(step => {
      const row = document.createElement('div');
      row.className = 'timeline-entry';
      row.innerHTML = `
        <div class="timeline-number">${step.step}</div>
        <div class="timeline-body">
          <div class="timeline-step-title">${step.title}</div>
          <div class="timeline-step-desc">${step.desc}</div>
        </div>
      `;
      timelineSteps.appendChild(row);
    });

    // Fallacies
    fallaciesList.innerHTML = '';
    data.fallacies.forEach(f => {
      const item = document.createElement('div');
      item.className = 'fallacy-item';
      item.textContent = f;
      fallaciesList.appendChild(item);
    });

    // Citations
    citationsList.innerHTML = '';
    data.citations.forEach(c => {
      const item = document.createElement('div');
      item.className = 'citation-item';
      item.innerHTML = `
        <i class="fa-solid fa-file-contract"></i>
        <a href="${c.url}" target="_blank" rel="noopener noreferrer">${c.title}</a>
      `;
      citationsList.appendChild(item);
    });
  }

  function animateGauge(targetScore) {
    const totalCircumference = 2 * Math.PI * 68; // ~427.25
    const targetOffset = totalCircumference - (totalCircumference * targetScore / 100);

    // Color based on score
    if (targetScore >= 80) {
      gaugeProgressCircle.style.stroke = '#059669'; // Emerald
    } else if (targetScore >= 45) {
      gaugeProgressCircle.style.stroke = '#D97706'; // Amber
    } else {
      gaugeProgressCircle.style.stroke = '#DC2626'; // Crimson
    }

    gaugeProgressCircle.style.strokeDasharray = `${totalCircumference}`;
    gaugeProgressCircle.style.strokeDashoffset = `${targetOffset}`;

    // Number roll-up counter
    let current = 0;
    const duration = 1000;
    const stepTime = 20;
    const steps = duration / stepTime;
    const increment = targetScore / steps;

    const counter = setInterval(() => {
      current += increment;
      if (current >= targetScore) {
        current = targetScore;
        clearInterval(counter);
      }
      truthScoreValue.textContent = `${Math.round(current)}%`;
    }, stepTime);
  }

  // -------------------------------------------------------------
  // COPY GONKA REQUEST IDS
  // -------------------------------------------------------------
  document.querySelectorAll('.id-copy-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      const targetId = btn.getAttribute('data-target');
      const valElem = document.getElementById(targetId);
      if (valElem) {
        navigator.clipboard.writeText(valElem.textContent.trim());
        showToast('Gonka Request ID copied to clipboard!');
      }
    });
  });

  // -------------------------------------------------------------
  // ACCESSIBILITY: TEXT-TO-SPEECH VERDICT
  // -------------------------------------------------------------
  ttsSpeakBtn.addEventListener('click', () => {
    if (!('speechSynthesis' in window)) {
      showToast('Speech synthesis not supported in this browser.');
      return;
    }

    if (isSpeaking) {
      window.speechSynthesis.cancel();
      isSpeaking = false;
      ttsBtnText.textContent = 'Listen to Verdict';
      return;
    }

    if (!activeVerificationResult) return;

    const textToRead = `VeritasGonka Fact Check Summary. Truth Score: ${activeVerificationResult.truthScore} percent. Status: ${activeVerificationResult.verdictLabel}. ${activeVerificationResult.headline}. ${activeVerificationResult.summary}. Multi-model cross examination routed through Gonka Router across DeepSeek V4, MiniMax M2.7, and Kimi K2.6.`;

    speechSynthUtterance = new SpeechSynthesisUtterance(textToRead);
    speechSynthUtterance.rate = 1.0;
    speechSynthUtterance.pitch = 1.0;

    speechSynthUtterance.onstart = () => {
      isSpeaking = true;
      ttsBtnText.textContent = 'Stop Audio';
    };

    speechSynthUtterance.onend = () => {
      isSpeaking = false;
      ttsBtnText.textContent = 'Listen to Verdict';
    };

    speechSynthUtterance.onerror = () => {
      isSpeaking = false;
      ttsBtnText.textContent = 'Listen to Verdict';
    };

    window.speechSynthesis.speak(speechSynthUtterance);
  });

  // -------------------------------------------------------------
  // COPY AUDIT TEXT
  // -------------------------------------------------------------
  copyShareLinkBtn.addEventListener('click', () => {
    if (!activeVerificationResult) return;
    const auditText = `[VeritasGonka Fact Check Proof]
Claim: "${activeVerificationResult.claim}"
Truth Score: ${activeVerificationResult.truthScore}% (${activeVerificationResult.verdictLabel})
Summary: ${activeVerificationResult.summary}
Cryptographic Gonka Audit Trail:
- DeepSeek V4: ${activeVerificationResult.deepseek.reqId}
- MiniMax M2.7: ${activeVerificationResult.minimax.reqId}
- Kimi K2.6: ${activeVerificationResult.kimi.reqId}
Audited via: https://gonkarouter.io/v1`;

    navigator.clipboard.writeText(auditText);
    showToast('Audit text copied to clipboard!');
  });

  // -------------------------------------------------------------
  // FACT CARD CERTIFICATE MODAL
  // -------------------------------------------------------------
  generateCertBtn.addEventListener('click', () => {
    if (!activeVerificationResult) return;
    certScoreNumber.textContent = `${activeVerificationResult.truthScore}%`;
    certVerdictTitle.textContent = activeVerificationResult.verdictLabel;
    certClaimText.textContent = `"${activeVerificationResult.claim}"`;
    certReq1.textContent = `DeepSeek V4: ${activeVerificationResult.deepseek.reqId}`;
    certReq2.textContent = `MiniMax M2.7: ${activeVerificationResult.minimax.reqId}`;
    certReq3.textContent = `Kimi K2.6: ${activeVerificationResult.kimi.reqId}`;
    certDate.textContent = `${new Date().toLocaleDateString()} ${new Date().toLocaleTimeString()} UTC`;

    factCardModal.style.display = 'flex';
  });

  closeFactCardModal.addEventListener('click', () => {
    factCardModal.style.display = 'none';
  });

  copyFactCardTextBtn.addEventListener('click', () => {
    copyShareLinkBtn.click();
    showToast('Fact card markdown copied!');
  });

  printFactCardBtn.addEventListener('click', () => {
    window.print();
  });

  // -------------------------------------------------------------
  // GONKA ROUTER CONFIG MODAL
  // -------------------------------------------------------------
  function openSettings() {
    initSettingsValues();
    settingsModal.style.display = 'flex';
  }

  openSettingsBtn.addEventListener('click', openSettings);
  if (footerConfigLink) footerConfigLink.addEventListener('click', (e) => { e.preventDefault(); openSettings(); });

  closeSettingsModal.addEventListener('click', () => {
    settingsModal.style.display = 'none';
  });

  toggleApiKeyVisibility.addEventListener('click', () => {
    if (settingApiKey.type === 'password') {
      settingApiKey.type = 'text';
      toggleApiKeyVisibility.innerHTML = '<i class="fa-regular fa-eye-slash"></i>';
    } else {
      settingApiKey.type = 'password';
      toggleApiKeyVisibility.innerHTML = '<i class="fa-regular fa-eye"></i>';
    }
  });

  saveSettingsBtn.addEventListener('click', () => {
    const inputKey = settingApiKey.value.trim();
    const existingPool = window.gonkaClient.config.apiKeyPool || [];
    if (inputKey && !existingPool.includes(inputKey)) {
      existingPool.unshift(inputKey);
    }
    window.gonkaClient.saveConfig({
      baseUrl: settingBaseUrl.value.trim() || 'https://gonkarouter.io/v1',
      apiKey: inputKey,
      apiKeyPool: existingPool,
      modelDeepseek: settingModel1.value.trim() || 'deepseek-ai/deepseek-v4',
      modelMinimax: settingModel2.value.trim() || 'minimax/minimax-m2.7',
      modelKimi: settingModel3.value.trim() || 'moonshot/kimi-k2.6'
    });
    settingsModal.style.display = 'none';
    showToast('Gonka Router settings saved successfully!');
  });

  resetSettingsBtn.addEventListener('click', () => {
    const def = window.gonkaClient.resetConfig();
    settingBaseUrl.value = def.baseUrl;
    settingApiKey.value = '';
    settingModel1.value = def.modelDeepseek;
    settingModel2.value = def.modelMinimax;
    settingModel3.value = def.modelKimi;
    showToast('Settings reset to default!');
  });

  testGonkaConnBtn.addEventListener('click', async () => {
    diagResult.textContent = 'Pinging Gonka Router endpoint...';
    const res = await window.gonkaClient.testConnection();
    diagResult.textContent = res.message;
  });

  function initSettingsValues() {
    const cfg = window.gonkaClient.config;
    settingBaseUrl.value = cfg.baseUrl;
    settingApiKey.value = cfg.apiKey || '';
    settingModel1.value = cfg.modelDeepseek;
    settingModel2.value = cfg.modelMinimax;
    settingModel3.value = cfg.modelKimi;
  }

  // -------------------------------------------------------------
  // PUBLIC AUDIT LEDGER
  // -------------------------------------------------------------
  function initLedger() {
    const raw = localStorage.getItem('veritas_ledger');
    let items = [];
    if (raw) {
      try { items = JSON.parse(raw); } catch (e) {}
    }
    renderLedgerRows(items);
  }

  function addLedgerEntry(result) {
    const raw = localStorage.getItem('veritas_ledger');
    let items = [];
    if (raw) {
      try { items = JSON.parse(raw); } catch (e) {}
    }

    const newEntry = {
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      claim: result.claim,
      truthScore: result.truthScore,
      statusClass: result.statusClass,
      verdictLabel: result.verdictLabel,
      reqIds: [result.deepseek.reqId, result.minimax.reqId, result.kimi.reqId]
    };

    items.unshift(newEntry);
    if (items.length > 20) items.pop();
    localStorage.setItem('veritas_ledger', JSON.stringify(items));
    renderLedgerRows(items);
  }

  function renderLedgerRows(items) {
    ledgerTableBody.innerHTML = '';
    items.forEach(it => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td>${it.timestamp}</td>
        <td class="table-claim" title="${it.claim}">${it.claim}</td>
        <td><strong>${it.truthScore}%</strong></td>
        <td><span class="stance-pill ${it.statusClass}">${it.verdictLabel}</span></td>
        <td><code class="table-req-chip">${it.reqIds[0]}</code></td>
        <td>
          <button class="editorial-btn text-btn compact recheck-btn" data-claim="${encodeURIComponent(it.claim)}">
            Verify Again
          </button>
        </td>
      `;
      ledgerTableBody.appendChild(tr);
    });

    document.querySelectorAll('.recheck-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const claim = decodeURIComponent(btn.getAttribute('data-claim'));
        claimInput.value = claim;
        tabTextClaim.click();
        triggerVerification(claim);
      });
    });
  }

  exportAuditLedgerBtn.addEventListener('click', () => {
    const raw = localStorage.getItem('veritas_ledger') || '[]';
    const blob = new Blob([raw], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `veritas_gonka_ledger_${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
    showToast('Audit ledger exported as JSON!');
  });

  // -------------------------------------------------------------
  // MULTILINGUAL SELECTOR
  // -------------------------------------------------------------
  languageSelect.addEventListener('change', (e) => {
    const lang = e.target.value;
    if (lang === 'es') {
      const sample = SAMPLE_CLAIMS.spanish;
      claimInput.value = sample.claim;
      tabTextClaim.click();
      triggerVerification(sample.claim);
      showToast('Idioma configurado a Español');
    } else {
      showToast(`Language switched to ${languageSelect.options[languageSelect.selectedIndex].text}`);
    }
  });

  // -------------------------------------------------------------
  // TOAST NOTIFICATIONS
  // -------------------------------------------------------------
  function showToast(msg) {
    const toast = document.createElement('div');
    toast.className = 'toast-message';
    toast.textContent = msg;
    toastContainer.appendChild(toast);
    setTimeout(() => {
      toast.style.opacity = '0';
      setTimeout(() => toast.remove(), 300);
    }, 3000);
  }

  // Close modals on escape key or backdrop click
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      factCardModal.style.display = 'none';
      settingsModal.style.display = 'none';
    }
  });

  [factCardModal, settingsModal].forEach(modal => {
    modal.addEventListener('click', (e) => {
      if (e.target === modal) modal.style.display = 'none';
    });
  });
});
