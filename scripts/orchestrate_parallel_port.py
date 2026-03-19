#!/usr/bin/env python3
"""
Orchestrator for parallel Mermaid diagram type porting using Warp ambient agents.

Usage:
    python scripts/orchestrate_parallel_port.py <diagram_type1> [diagram_type2 ...]

Example:
    python scripts/orchestrate_parallel_port.py architecture packet
"""

import argparse
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime
from pathlib import Path
from threading import Lock
from typing import Dict, List, Optional


class AgentTask:
    """Represents a single ambient agent task."""
    
    def __init__(self, diagram_type: str, task_id: Optional[str] = None):
        self.diagram_type = diagram_type
        self.task_id = task_id
        self.branch_name = f"zach/mermaid-port-{diagram_type}"
        self.status = "pending"
        self.launched_at = None
        self.completed_at = None
        self.session_url = None
    
    def to_dict(self):
        return {
            "diagram_type": self.diagram_type,
            "task_id": self.task_id,
            "branch_name": self.branch_name,
            "status": self.status,
            "launched_at": self.launched_at,
            "completed_at": self.completed_at,
            "session_url": self.session_url,
        }
    
    @classmethod
    def from_dict(cls, data: dict):
        task = cls(data["diagram_type"], data.get("task_id"))
        task.branch_name = data["branch_name"]
        task.status = data["status"]
        task.launched_at = data.get("launched_at")
        task.completed_at = data.get("completed_at")
        task.session_url = data.get("session_url")
        return task


class Orchestrator:
    """Orchestrates parallel ambient agent execution."""
    
    def __init__(self, diagram_types: List[str], environment_id: str = "SVhg783GBFQHk1OfdPfFU9"):
        self.diagram_types = diagram_types
        self.environment_id = environment_id
        self.tasks: List[AgentTask] = [AgentTask(dt) for dt in diagram_types]
        self.metadata_file = Path("orchestrator_state.json")
        self.state_lock = Lock()  # Thread-safe state updates
    
    def save_state(self):
        """Save current state to JSON file (thread-safe)."""
        with self.state_lock:
            state = {
                "environment_id": self.environment_id,
                "tasks": [task.to_dict() for task in self.tasks],
                "last_updated": datetime.now().isoformat(),
            }
            self.metadata_file.write_text(json.dumps(state, indent=2))
            print(f"💾 State saved to {self.metadata_file}")
    
    def load_state(self):
        """Load state from JSON file if it exists."""
        if self.metadata_file.exists():
            state = json.loads(self.metadata_file.read_text())
            self.tasks = [AgentTask.from_dict(t) for t in state["tasks"]]
            self.environment_id = state.get("environment_id", self.environment_id)
            print(f"📂 State loaded from {self.metadata_file}")
            return True
        return False
    
    def push_base_branch(self):
        """Push the current branch to GitHub."""
        print("\n📤 Pushing base branch to GitHub...")
        try:
            result = subprocess.run(
                ["git", "push", "origin", "zach/mermaid-port"],
                capture_output=True,
                text=True,
                check=True,
            )
            print("✅ Base branch pushed successfully")
            return True
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to push base branch: {e.stderr}")
            return False
    
    def launch_agent(self, task: AgentTask) -> bool:
        """Launch an ambient agent for a specific diagram type."""
        prompt = f"""You are working on the mermaid_to_svg Mermaid 11.6.0 port.
Your assigned diagram type: {task.diagram_type}

First, navigate to the project:
1. cd /workspace/warp-internal
2. git fetch origin
3. git checkout zach/mermaid-port
4. git pull origin zach/mermaid-port
5. cd mermaid_to_svg

Then read and follow the complete instructions in .claude/skills/ambient-agent-diagram-type-instructions.md for the full workflow."""
        
        print(f"\n🚀 Launching agent for '{task.diagram_type}'...")
        
        try:
            cmd = [
                "warp-dev", "agent", "run-ambient",
                "--prompt", prompt,
                "--environment", self.environment_id,
                "--name", f"mermaid-port-{task.diagram_type}",
                "--output-format", "json",
            ]
            
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            
            # Parse output to extract task ID and session URL
            try:
                output = json.loads(result.stdout)
                task.task_id = output.get("task_id") or output.get("id")
                task.session_url = output.get("session_url") or output.get("url")
            except json.JSONDecodeError:
                # Parse text output if JSON parsing fails
                import re
                run_id_match = re.search(r"run ID: ([a-f0-9-]+)", result.stdout)
                session_match = re.search(r"View agent session: (https://[^\s]+)", result.stdout)
                
                if run_id_match:
                    task.task_id = run_id_match.group(1)
                if session_match:
                    task.session_url = session_match.group(1)
            
            task.status = "running"
            task.launched_at = datetime.now().isoformat()
            
            if task.task_id:
                print(f"✅ Agent launched: {task.task_id}")
            else:
                print(f"✅ Agent launched (ID not captured)")
            
            if task.session_url:
                print(f"   Session: {task.session_url}")
            
            return True
                
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to launch agent: {e.stderr}")
            task.status = "failed"
            return False
    
    def launch_all(self, max_workers: int = 5):
        """Launch all pending agents in parallel.
        
        Args:
            max_workers: Maximum number of concurrent agent launches (default: 5)
        """
        pending_tasks = [task for task in self.tasks if task.status == "pending"]
        
        if not pending_tasks:
            print("\n⚠️  No pending tasks to launch")
            return
        
        print(f"\n{'='*60}")
        print(f"🎯 Launching {len(pending_tasks)} ambient agents in parallel")
        print(f"   Max concurrent launches: {max_workers}")
        print(f"{'='*60}")
        
        # Launch agents in parallel using ThreadPoolExecutor
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            # Submit all launch tasks
            future_to_task = {executor.submit(self.launch_agent, task): task for task in pending_tasks}
            
            # Process results as they complete
            for future in as_completed(future_to_task):
                task = future_to_task[future]
                try:
                    success = future.result()
                    # Save state after each launch completes
                    self.save_state()
                except Exception as e:
                    print(f"❌ Exception launching agent for '{task.diagram_type}': {e}")
                    task.status = "failed"
                    self.save_state()
        
        print(f"\n{'='*60}")
        print(f"✅ All agents launched")
        print(f"{'='*60}\n")
    
    def check_task_status(self, task: AgentTask) -> str:
        """Check the status of a task using REST API."""
        if not task.task_id:
            return "unknown"
        
        try:
            # Get API key from environment or use warp-dev CLI
            api_key = os.environ.get("WARP_API_KEY")
            
            # Use curl to query the API
            cmd = [
                "curl", "-s", "-X", "GET",
                f"https://staging.warp.dev/api/v1/agent/tasks/{task.task_id}",
            ]
            
            if api_key:
                cmd.extend(["-H", f"Authorization: Bearer {api_key}"])
            
            result = subprocess.run(cmd, capture_output=True, text=True)
            
            if result.returncode == 0:
                try:
                    data = json.loads(result.stdout)
                    state = data.get("state", "unknown").lower()
                    
                    # Map API states to our status
                    if state in ["completed", "success"]:
                        return "completed"
                    elif state in ["failed", "error"]:
                        return "failed"
                    elif state in ["running", "inprogress", "in_progress"]:
                        return "running"
                    else:
                        return state
                except json.JSONDecodeError:
                    return task.status
            else:
                return task.status
                
        except Exception as e:
            print(f"⚠️  Error checking status for {task.task_id}: {e}")
            return task.status
    
    def monitor_loop(self, interval: int = 30):
        """Monitor agent progress in a loop."""
        print(f"\n👀 Monitoring agents (polling every {interval}s)...")
        print("Press Ctrl+C to stop monitoring\n")
        
        try:
            while True:
                self.display_status()
                
                # Check if all agents are done
                statuses = [task.status for task in self.tasks]
                if all(s in ["completed", "failed"] for s in statuses):
                    print("\n🎉 All agents finished!")
                    break
                
                time.sleep(interval)
                
                # Update task statuses
                for task in self.tasks:
                    if task.status == "running":
                        new_status = self.check_task_status(task)
                        if new_status != task.status:
                            task.status = new_status
                            if new_status in ["completed", "failed"]:
                                task.completed_at = datetime.now().isoformat()
                            self.save_state()
        
        except KeyboardInterrupt:
            print("\n\n⏸️  Monitoring paused. State saved.")
            self.save_state()
    
    def display_status(self):
        """Display current status of all agents."""
        print(f"\n{'='*80}")
        print(f"Status at {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"{'='*80}")
        
        # Table header
        print(f"{'Type':<20} {'Status':<12} {'Branch':<35} {'Task ID':<15}")
        print(f"{'-'*20} {'-'*12} {'-'*35} {'-'*15}")
        
        for task in self.tasks:
            status_emoji = {
                "pending": "⏳",
                "running": "🏃",
                "completed": "✅",
                "failed": "❌",
                "unknown": "❓",
            }.get(task.status, "❓")
            
            task_id_display = (task.task_id or "N/A")[:15]
            
            print(f"{task.diagram_type:<20} {status_emoji} {task.status:<10} {task.branch_name:<35} {task_id_display:<15}")
        
        if any(task.session_url for task in self.tasks):
            print("\nSession URLs:")
            for task in self.tasks:
                if task.session_url:
                    print(f"  {task.diagram_type}: {task.session_url}")
        
        print()
    
    def fetch_branches(self):
        """Fetch all agent branches from GitHub."""
        print("\n📥 Fetching branches from GitHub...")
        try:
            subprocess.run(["git", "fetch", "origin"], check=True)
            print("✅ Branches fetched")
            
            print("\nAgent branches:")
            for task in self.tasks:
                result = subprocess.run(
                    ["git", "branch", "-r", "--list", f"origin/{task.branch_name}"],
                    capture_output=True,
                    text=True,
                )
                if result.stdout.strip():
                    print(f"  ✅ {task.branch_name}")
                else:
                    print(f"  ❌ {task.branch_name} (not found)")
        except subprocess.CalledProcessError as e:
            print(f"❌ Failed to fetch branches: {e}")


def main():
    parser = argparse.ArgumentParser(
        description="Orchestrate parallel Mermaid diagram type porting with ambient agents"
    )
    parser.add_argument(
        "diagram_types",
        nargs="*",
        help="Diagram types to process (e.g., architecture packet)",
    )
    parser.add_argument(
        "--environment",
        default="SVhg783GBFQHk1OfdPfFU9",
        help="Warp Dev environment ID",
    )
    parser.add_argument(
        "--resume",
        action="store_true",
        help="Resume from saved state",
    )
    parser.add_argument(
        "--status",
        action="store_true",
        help="Show status only (don't launch)",
    )
    parser.add_argument(
        "--monitor",
        action="store_true",
        help="Monitor running agents",
    )
    parser.add_argument(
        "--fetch",
        action="store_true",
        help="Fetch completed branches from GitHub",
    )
    parser.add_argument(
        "--max-workers",
        type=int,
        default=5,
        help="Maximum number of concurrent agent launches (default: 5)",
    )
    
    args = parser.parse_args()
    
    # Initialize orchestrator
    orchestrator = None
    
    if args.resume or args.status or args.monitor or args.fetch:
        orchestrator = Orchestrator([], args.environment)
        if not orchestrator.load_state():
            print("❌ No saved state found. Run with diagram types to start new orchestration.")
            sys.exit(1)
    else:
        if not args.diagram_types:
            parser.print_help()
            sys.exit(1)
        orchestrator = Orchestrator(args.diagram_types, args.environment)
    
    # Execute requested action
    if args.status:
        orchestrator.display_status()
    elif args.monitor:
        orchestrator.monitor_loop()
    elif args.fetch:
        orchestrator.fetch_branches()
    else:
        # Full launch sequence
        if not orchestrator.push_base_branch():
            print("❌ Failed to push base branch. Aborting.")
            sys.exit(1)
        
        orchestrator.launch_all(max_workers=args.max_workers)
        orchestrator.save_state()
        
        print("\nNext steps:")
        print("  1. Monitor progress: python scripts/orchestrate_parallel_port.py --monitor")
        print("  2. Check status: python scripts/orchestrate_parallel_port.py --status")
        print("  3. Fetch branches: python scripts/orchestrate_parallel_port.py --fetch")


if __name__ == "__main__":
    main()
