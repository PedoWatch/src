# PedoWatch Project Plan - Updated with Backend Development Roadmap

---

## **Overview**

The **PedoWatch Project** is a multi-platform child protection system designed to monitor online communities and games such as **Discord, Roblox, and Reddit**, detecting harmful or predatory behaviors in real-time. The system integrates **heuristic keyword analysis**, **AI inference**, and **semantic NLP models** (utilizing smaller models optimized for cost-effectiveness).

The system introduces **multiple AI models**, combining LLMs and smaller, faster models for cost optimization. It implements **multi-level detection** for precise threat assessment (Level 0-5) and introduces community feedback loops to refine detections over time.

---

## **Tech Stack**

### **Backend Infrastructure**
1. **Rust**: High-performance, scalable backend server with **Actix Web**.
2. **MongoDB**: Reliable and efficient document-based database for storing logs and analysis.
3. **OpenAI GPT/Embeddings**: For semantic analysis and advanced natural language processing.
4. **Smaller LLMs**:
   - **DistilBERT**: Faster, smaller transformer model suitable for real-time analysis.
   - **ALBERT (A Lite BERT)**: Lightweight, optimized NLP model for resource-constrained tasks.
   - **RoBERTa (Small Variants)**: Optimized versions of the RoBERTa language model to balance cost vs. performance.

### **Consumer Integration**
1. **Discord Integration**:
   - Self-hostable Discord bot monitoring server messages.
2. **Roblox Integration**:
   - Lua script relay using `HTTPService` for sending chat data securely.
3. **Reddit Integration**:
   - **PRAW API** for message ingestion and analysis.

---

## **Improved Keyword Heuristic List**

Comprehensive list of over **200 keywords** for heuristic filtering and detection, expanding on behaviors indicative of grooming and predatory activity.

---

## **AI Model Stack**

### Semantic Models
- GPT (OpenAI API calls)  
- DistilBERT  
- ALBERT  
- RoBERTa  

The models will work in conjunction with each other in an ensemble fashion to ensure diverse detection without relying solely on a single large LLM.

---

## **Backend Development Roadmap**

### **Phase 1: Set up Development Infrastructure**
1. **Define API Contracts**:
   - REST endpoints for AI inference, consumer bot interaction (Discord/Roblox/Reddit).
   - Endpoint example: `/analyze` for submitting chat messages for analysis.
2. **Set up MongoDB Instance**:
   - MongoDB will store logs, user reports, analysis results, and heuristic keyword hits.
3. **Establish Local Development Environment**:
   - Containerize with Docker.
   - Set up development server with Rust + Actix Web.

### **Phase 2: Model Integration Pipeline**
1. **API Layer for OpenAI GPT Inference Calls**:
   - Implement endpoints to submit chat content for analysis.
2. **Embed DistilBERT/ALBERT for Faster Analysis**:
   - Allow fallback to smaller, faster models depending on load.
3. **Set up Real-time Message Processing**:
   - Continuous ingestion pipeline for incoming Discord/Roblox/Reddit messages.

### **Phase 3: Implement Multi-Level Threat Detection**
1. **Define Detection Levels**:
   - Level 0 (normal text) through Level 5 (extremely high likelihood of grooming/predatory intent).
2. **Heuristic + Semantic Analysis Pipeline**:
   - Combine keyword heuristics with embeddings and NLP in multi-tiered scoring.

### **Phase 4: Consumer Bot Integration**
1. **Discord Bot**:
   - Use `discord.py` or a Rust-based bot.
   - Monitor server messages in real-time and send analysis to the backend.
2. **Roblox HTTP Integration**:
   - Lua scripts to pass Roblox chat messages via HTTP requests securely to the server.
3. **Reddit Monitoring**:
   - Leverage the **PRAW API** to monitor subreddits for harmful trends.

### **Phase 5: Testing**
1. Unit tests on Rust inference endpoints.
2. Simulation of user interactions to verify threat detection accuracy.
3. Security testing to ensure secure communication between models and endpoints.

---

## **Full AI Model System**
The AI inference system uses the following design:

1. Input message → Heuristic filtering against 200+ keywords.
2. NLP semantic analysis against GPT or DistilBERT embeddings.
3. Levels assigned via heuristic scoring and semantic confidence scores.
4. Return response with a threat level (Level 0-5).

---

## **Deployment Plan**

### **Production Stack**
1. Use **AWS or DigitalOcean** for scalable hosting.
2. Use **MongoDB Atlas** for database clustering.
3. Use **Rust server endpoints + GraphQL/REST** API for message handling.

### **Monitoring**
- CloudWatch or similar for monitoring server health.
- Alerts on threshold levels detected in communities.

---

## **Consumer Integration**

### Discord
- A **self-hostable bot** will monitor messages and integrate with backend services.

### Roblox
- Roblox HTTPService will submit chat logs securely to the server via an endpoint.

### Reddit
- Using the **PRAW API**, monitor threads and comment streams for predatory behavior.

---

## **Future Roadmap**
1. **User Feedback Loop**:
   - Allow community members to submit false positives or flagged content reports.
   - Use this feedback to retrain models.
2. **Expand Platform Support**:
   - Explore other online platforms beyond Discord, Roblox, and Reddit.
3. **Privacy Compliance**:
   - Ensure compliance with GDPR and COPPA.
   - Anonymize data while ensuring protection detection remains intact.

---

## **Updated Project Stack**

### Backend
- **Rust + Actix Web** for real-time inference endpoints.
- MongoDB for efficient log storage.
- Smaller ML inference pipelines using DistilBERT/ALBERT alongside GPT embeddings.
  
### Consumer Integration
- Discord bot (self-hostable).
- Roblox HTTP Integration via Lua scripts.
- Reddit integration via **PRAW** API.

---

## **Next Steps**
1. Develop endpoints and integrate smaller models for testing detection accuracy.
2. Build out the Discord Bot pipeline for real-time message relay.
3. Finalize data heuristics and NLP model integration.

**Estimated Timeline: 6 - 12 months for full MVP.**

---

End of PedoWatch Plan.
