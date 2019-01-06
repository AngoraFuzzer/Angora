/*
  Make optimization fail for branches
  e.g
  if (x == 1 & y == 1) {}
  =>
  if (x==1) {
    if (y == 1) {}
  }
 */

#include "./config.h"
#include "./debug.h"

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "llvm/ADT/Statistic.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/IntrinsicInst.h"
#include "llvm/Support/Debug.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"
#include "llvm/ADT/SmallSet.h"
#include "llvm/Support/raw_ostream.h"
#include "llvm/IR/DebugInfo.h"
using namespace llvm;

namespace {

  class UnfoldBranch : public FunctionPass {
  private:
    Type *VoidTy;
    IntegerType *Int8Ty;
    IntegerType *Int32Ty;

    Constant *UnfoldBranchFn;
  public:

    static char ID;

    UnfoldBranch() : FunctionPass(ID) { }

    bool doInitialization(Module &M) override;
    bool doFinalization(Module &M) override;
    bool runOnFunction(Function &F) override;
 };

}

char UnfoldBranch::ID = 0;

bool UnfoldBranch::doInitialization(Module &M) {

  LLVMContext &C = M.getContext();

  Int8Ty  = IntegerType::getInt8Ty(C);
  Int32Ty = IntegerType::getInt32Ty(C);
  VoidTy = Type::getVoidTy(C);

  srandom(1851655);

  UnfoldBranchFn =
     M.getOrInsertFunction("__unfold_branch_fn", VoidTy, Int32Ty, nullptr);

  return true;
}

bool UnfoldBranch::doFinalization(Module &M) {

  return true;
}

bool UnfoldBranch::runOnFunction(Function &F) {

  // if the function is declaration, ignore
  if (F.isDeclaration()) return false;
#ifndef ENABLE_UNFOLD_BRANCH
  return false;
#endif
  SmallSet<BasicBlock*, 20> VisitedBB;
  LLVMContext& C = F.getContext();
  for (auto &BB : F) {
    // for (BasicBlock::iterator I = BB.begin(); I != BB.end(); I++) {
    //   Instruction *Inst = &(*I);

    Instruction *Inst = BB.getTerminator();
    if (isa<BranchInst>(Inst)) {

      BranchInst *BI = dyn_cast<BranchInst>(Inst);

      if (BI->isUnconditional() || BI->getNumSuccessors() < 2)
        continue;

      Value *Cond = BI->getCondition();
      if (!Cond) continue;

      if (BI->getNumSuccessors() > 0 ) {
        BasicBlock* B0 = BI->getSuccessor(0);
        if (B0 && VisitedBB.count(B0) == 0) {
          VisitedBB.insert(B0);
          BasicBlock::iterator IP = B0->getFirstInsertionPt();
          IRBuilder<> IRB(&(*IP));
          unsigned int cur_loc = RRR(MAP_SIZE);
          CallInst* Call = IRB.CreateCall(UnfoldBranchFn, {ConstantInt::get(Int32Ty, cur_loc)});
          Call->setMetadata(C.getMDKindID("unfold"), MDNode::get(C, None));
        }
      }

      if (BI->getNumSuccessors() > 1 ) {
        BasicBlock* B1 = BI->getSuccessor(1);
        if (B1 && VisitedBB.count(B1) == 0) {
          VisitedBB.insert(B1);
          BasicBlock::iterator IP = B1->getFirstInsertionPt();
          IRBuilder<> IRB(&(*IP));
          unsigned int cur_loc = RRR(MAP_SIZE);
          CallInst* Call = IRB.CreateCall(UnfoldBranchFn, {ConstantInt::get(Int32Ty, cur_loc)});
          Call->setMetadata(C.getMDKindID("unfold"), MDNode::get(C, None));
        }
      }
    }

    /*
    if (isa<ReturnInst>(Inst)) {
      ReturnInst *RI = dyn_cast<ReturnInst>(Inst);
      Value* Ret = RI->getReturnValue();
      if (Ret && isa<SExtInst>(Ret)) {
        // errs() << "ret: " << *Ret << "\n";
        // abort();
        // F.addAttribute(AttributeSet::ReturnIndex, Attribute::SExt);
      }
      // errs() << F.getAttribute(AttributeSet::ReturnIndex, Attribute::SExt).getValueAsInt() <<"\n";
    }
    */
  }
  return true;

}


static void registerUnfoldBranchPass(const PassManagerBuilder &,
                                     legacy::PassManagerBase &PM) {

  PM.add(new UnfoldBranch());

}

static RegisterPass<UnfoldBranch> X("unfold_branch_pass", "Unfold Branch Pass");


static RegisterStandardPasses RegisterAFLPass(PassManagerBuilder::EP_EarlyAsPossible, registerUnfoldBranchPass);

/*
static RegisterStandardPasses RegisterAFLPass0(
    PassManagerBuilder::EP_EnabledOnOptLevel0, registerAFLPass);
*/
