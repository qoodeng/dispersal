#### R Code Associated with "Hunter-gatherer residential mobility and the marginal value of rainforest patches" by Vivek V. Venkataraman, Thomas S. Kraft, Nathaniel J. Dominy, and Kirk M. Endicott

# load required R packages (if packages are not yet installed, run install.packages(c("ggplot2", "reshape2", "nls2", "rootSolve", "plyr", "dplyr", "RCurl"))
library(ggplot2)
library(reshape2)
library(nls2)
library(rootSolve)
library(plyr)
library(dplyr)
library(curl)
library(AICcmodavg)

url1 <- "https://github.com/ThomasKraft/HunterGathererMarginalValueTheorem/raw/master/Data/campmovementdata.RData"
load(curl(url1))

# View the structure of the main data frame
str(tot)

# Remove camp 7. Data from this camp was used in the calculation of environmental averages but cannot be used to construct gain functions because there is uncertainty regarding the dates of occupation.
tot<-filter(tot, Camp.number != 7)
tot$Camp.number <- factor(tot$Camp.number) #re-factor to remove extra level

########### ########### ########### ########### ########### ###########
#Functions to predict optimal patch residence times according to the marginal value theorem #####
########### ########### ########### ########### ########### ###########

######### Asymptotic exponential function ###########
AE<-function(y,x,Avg){
  AE_a=2000000
  AE_b=2
  result<- list(AE_a = NA, AE_b = NA, AE_AIC = NA, expected.stay_AE=NA)
  AE_mod<-try(nls(y~AE_a*(1-exp(-AE_b*x)),start=list(AE_a = AE_a, AE_b = AE_b)),silent=T)
  if(class(AE_mod)=="try-error") {
    return(result)
  }
  else{
    AE_a = coef(AE_mod)[1]
    AE_b = coef(AE_mod)[2]
    g<- function(x){}
    body(g) <- D(substitute(AE_a*(1-exp(-AE_b*x)) - Avg*x,list(AE_a = AE_a, AE_b = AE_b, Avg= Avg)), "x")
    try(expected.stay_AE <- uniroot.all(g,c(1,100)), silent=T)
    if(length(expected.stay_AE) == 0)  {
      return(list(AE_a =coef(AE_mod)[1] , AE_b = coef(AE_mod)[2], AE_AIC = AICc(AE_mod), expected.stay_AE=NA, model=AE_mod))                                             }
    else {
      g2<-function(x){}
      body(g2) <- D(D(substitute(AE_a*(1-exp(-AE_b*x))-Avg*x,list(AE_a = AE_a, AE_b = AE_b, Avg = Avg)), "x"),"x")
      expected.stay_AE<-expected.stay_AE[which(g2(expected.stay_AE)<0)]
      result <-list(AE_a = coef(AE_mod)[1], AE_b = coef(AE_mod)[2], AE_AIC = AICc(AE_mod), expected.stay_AE = expected.stay_AE, model=AE_mod)
      return(result)
    }
  }
}

###### Michaelis Menton function ######
MM<-function(y,x, Avg){
  MM_a = 2000000
  MM_c = 0.0005
  result<- list(MM_a = NA, MM_c = NA, MM_AIC = NA, expected.stay_MM=NA)
  MM_mod<-try(nls(y~MM_a*x/(MM_c+x),start=list(MM_a = MM_a, MM_c = MM_c)),silent=T)
  if(class(MM_mod)=="try-error") {
    return(result)
  }
  else{
    MM_a = coef(MM_mod)[1]
    MM_c = coef(MM_mod)[2]
    g<- function(x){}
    body(g) <- D(substitute(MM_a*x/(MM_c + x)-Avg*x,list(MM_a = MM_a,MM_c = MM_c)), "x")
    
    try(expected.stay_MM <- uniroot.all(g,c(1,100)), silent=T)
    if(length(expected.stay_MM) == 0)  {
      return(list(MM_a =coef(MM_mod)[1] , MM_c = coef(MM_mod)[2], MM_AIC = AICc(MM_mod), expected.stay_MM=NA, model=MM_mod))                                             }
    else{
      g2<-function(x){}
      body(g2) <- D(D(substitute(MM_a*x/(MM_c + x)-Avg*x,list(MM_a = MM_a,MM_c = MM_c)), "x"),"x")
      expected.stay_MM<-expected.stay_MM[which(g2(expected.stay_MM)<0)]
      result <-list(MM_a = coef(MM_mod)[1], MM_c = coef(MM_mod)[2], MM_AIC = AICc(MM_mod), expected.stay_MM = expected.stay_MM, model=MM_mod)
      return(result)
    }
    
  }
}

######### Holling Type III function ############
H3<-function(y,x,Avg){
  H3_a = 2000000
  H3_b = 5
  result<- list(H3_a = NA, H3_b = NA, H3_AIC=NA, expected.stay_H3=NA)
  H3_mod<-try(nls(y~H3_a*x^2/(H3_b^2+x^2),start=list(H3_a = H3_a,H3_b = H3_b)),silent=T)
  if(class(H3_mod)=="try-error") {
    return(result)
  }
  else{
    H3_a = coef(H3_mod)[1]
    H3_b = coef(H3_mod)[2]
    g<- function(x){}
    body(g) <- D(substitute(H3_a*x^2/(H3_b^2+x^2)-Avg*x,list(H3_a = H3_a, H3_b = H3_b,Avg=Avg)), "x")
    try(expected.stay_H3 <- uniroot.all(g,c(1,100)), silent=T)
    if(length(expected.stay_H3) == 0)  {
      return(list(H3_a =coef(H3_mod)[1] , H3_b = coef(H3_mod)[2], H3_AIC = AICc(H3_mod), expected.stay_H3=NA, model=H3_mod))
    }
    else{
      g2<-function(x){}
      body(g2) <- D(D(substitute(H3_a*x^2/(H3_b^2+x^2)-Avg*x,list(H3_a = H3_a, H3_b = H3_b, Avg=Avg)), "x"),"x")
      expected.stay_H3<-expected.stay_H3[which(g2(expected.stay_H3)<0)]
      result <-list(H3_a = coef(H3_mod)[1], H3_b = coef(H3_mod)[2], H3_AIC = AICc(H3_mod), expected.stay_H3 = expected.stay_H3, model=H3_mod)
      return(result)
    }
  }
}

######## Linear function #########
LM<-function(y,x,Avg){
  lm_mod<-lm(y~x-1)
  result<-list(LM_m=coef(lm_mod)[1],LM_AIC=AICc(lm_mod), model=lm_mod)
  return(result)
}

############ Sigmoidal function: 4-parameter logistic ###########
SIG<-function(y,x,Avg){
  sig_a=30000
  sig_b=1
  sig_c=5
  sig_d=1.5
  result<- list(sig_a = NA, sig_b = NA,sig_c = NA, sig_d=NA, sig_AIC = NA, expected.stay_SIG=NA)
  sig_mod <- try(nls(y~SSfpl(x,a,b,c,d)), silent=T) 
  if(class(sig_mod)=="try-error") {
    return(result)
  }
  else{
    sig_a = coef(sig_mod)[1]
    sig_b = coef(sig_mod)[2]
    sig_c = coef(sig_mod)[3]
    sig_d = coef(sig_mod)[4]
    g<- function(x){}
    body(g) <- D(substitute((sig_a + (sig_b - sig_a)/(1+exp((sig_c - x)/sig_d)))-Avg*x,list(sig_a=sig_a,sig_b=sig_b,sig_c=sig_c,sig_d=sig_d, Avg=Avg)), "x")
    try(expected.stay_SIG <- uniroot.all(g,c(1,100)), silent=T)
    if(length(expected.stay_SIG) == 0)  {
      return(list(sig_a =coef(sig_mod)[1] , sig_b = coef(sig_mod)[2], sig_c = coef(sig_mod)[3],sig_d = coef(sig_mod)[4], sig_AIC = AICc(sig_mod), expected.stay_SIG=NA, model=sig_mod))
    }
    else{
      g2<-function(x){}
      body(g2) <- D(D(substitute((sig_a + (sig_b - sig_a)/(1+exp((sig_c - x)/sig_d)))-Avg*x,list(sig_a=sig_a,sig_b=sig_b,sig_c=sig_c,sig_d=sig_d, Avg=Avg)), "x"),"x")
      expected.stay_SIG<-expected.stay_SIG[which(g2(expected.stay_SIG)<0)]
      result <-list(sig_a = coef(sig_mod)[1], sig_b = coef(sig_mod)[2],sig_c = coef(sig_mod)[3],sig_d = coef(sig_mod)[4], sig_AIC = AICc(sig_mod), expected.stay_SIG = expected.stay_SIG, model=sig_mod)
      return(result)
    }
  }
}

### End MVT functions section ###


###### Apply all of the models to the resource set-camp combinations and save the important parameters. Also save AIC values from different model fits for use in model selection later. ########

out <- tot %>%
  group_by(Camp.number, variable) %>%
  summarise(
    sig_a=SIG(value,count,mean(Avg))$sig_a,
    sig_b=SIG(value,count,mean(Avg))$sig_b,
    sig_c=SIG(value,count,mean(Avg))$sig_c,
    sig_d=SIG(value,count,mean(Avg))$sig_d,
    SIG=SIG(value,count,mean(Avg))$sig_AIC,
    expected.stay_SIG=SIG(value,count,mean(Avg))$expected.stay_SIG,
    diffST_SIG = abs(expected.stay_SIG-length(value)),
    LM_m=LM(value,count,mean(Avg))$LM_m,
    LM=LM(value,count,mean(Avg))$LM_AIC,
    H3_a=H3(value,count,mean(Avg))$H3_a,
    H3_b=H3(value,count,mean(Avg))$H3_b,
    H3=H3(value,count,mean(Avg))$H3_AIC, 
    expected.stay_H3=H3(value,count,mean(Avg))$expected.stay_H3,
    diffST_H3 = abs(expected.stay_H3-length(value)),
    MM_a=MM(value,count,mean(Avg))$MM_a,
    MM_c=MM(value,count,mean(Avg))$MM_c,
    MM=MM(value,count,mean(Avg))$MM_AIC, 
    expected.stay_MM=MM(value,count,mean(Avg))$expected.stay_MM, 
    diffST_MM = abs(expected.stay_MM-length(value)),
    AE_a=AE(value,count,mean(Avg))$AE_a,
    AE_b=AE(value,count,mean(Avg))$AE_b,
    AE=AE(value,count,mean(Avg))$AE_AIC,
    expected.stay_AE=AE(value,count,mean(Avg))$expected.stay_AE,
    diffST_AE = abs(expected.stay_AE-length(value)),
    actual=length(value)-1, 
    Avg=mean(Avg)
    ) %>%
  rowwise() %>%   #determine which model is the best using AIC
  mutate(best_mod = which.min(c(SIG, LM, H3, MM, AE))) %>%
  mutate(best_mod = plyr::mapvalues(best_mod, from=c(1,2,3,4,5), to=c("SIG", "LM", "H3", "MM", "AE"), warn_missing= FALSE)) %>%
  as.data.frame()


# Replace any infinite values (indicating model did not fit) with NA
out <- do.call(data.frame,lapply(out, function(x) replace(x, is.infinite(x),NA)))

#remove rows for which no models could be fit
out <- out[-which(is.na(out$SIG) & is.na(out$LM) & is.na(out$H3) & is.na(out$MM) & is.na(out$AE)), ] 


##### Produce results in table 1
# Start by only selecting relevant columns
out2 <- out %>% select(Camp.number, variable, diffST_SIG,diffST_H3,diffST_MM,diffST_AE,actual, best_mod)

# Camps 1, 4, and 8 must be removed because we can't claim that the actual functional form is informative. Don't run this step if you want to see functional forms for those camps.
out2 <- filter(out2,!Camp.number %in% c(1,4,8))


## Calculate what proportion of fits are depleting versus non-depleting for different resource sets
types <- out2 %>%
  group_by(variable, best_mod) %>%
  summarize(numb=n())
# note that "types" contains the information in table 1

#calculate proportion depleting for each resource category (last column of table 1)
types %>%
  group_by(variable) %>%
  summarize(prop.depleting = sum(numb[which(best_mod != "LM")])/sum(numb))

#calculate the overall proportion that are depleting, proportion sigmoid, and proportion asymptotic
types2 <- types %>%
  group_by(best_mod) %>%
  summarize(Sums = sum(numb)) %>%
  mutate(prop= Sums/sum(Sums))

prop.depleting <- sum(types2[types2$best_mod != "LM",]$prop); print(prop.depleting)
prop.sigmoid <- sum(types2[types2$best_mod == "SIG" | types2$best_mod == "H3",]$prop); print(prop.sigmoid)
prop.asymptotic <- sum(types2[types2$best_mod == "MM" | types2$best_mod == "AE",]$prop); print(prop.asymptotic)


# The following for-loop extracts the expected residence time calculated by the MVT for the best fitting model. Note that expected residence time is set to NA for non-depleting (linear) models.
set <- select(out, expected.stay_SIG,expected.stay_H3,expected.stay_MM, expected.stay_AE)
  
out$b<-NA
for(i in 1:length(out$best_mod)){
  if(out$best_mod[i] == "LM"){
    out$b[i] <- NA
    
  }
  else{
    out$b[i] <- set[i, which(grepl(out$best_mod[i], names(set)))]
  }
}

# reduce the dataset to include only the necessary variables for final analysis
final.df <- select(out, Camp.number, variable, actual, best_mod, b)
final.df <- na.omit(final.df)

############ PLOT FIGURE 2 ########
### extract model objects from all of the best fit models for plotting deterministic functions
modlist <- list()
for(i in c(1:nrow(out))){
  camp <- out$Camp.number[i] 
  resource <- out$variable[i] 
  model <- out$best_mod[i] 
  
  sub.dat <- tot[tot$Camp.number == camp & tot$variable == resource,]
  runmod <- do.call(as.character(model), args=list(y=sub.dat$value, x=sub.dat$count, Avg=mean(sub.dat$Avg)))
  
  modlist[[i]] = runmod$model
}

#generate predicted values from each model (for plotting the deterministic functions)
xvals<-seq(min(tot$count), max(tot$count), .1)
preds <- ldply(modlist, function(mod) {
  yy = predict(mod, data.frame(x=xvals))
})

#transpose the df and then melt it so that we have a single column of values and a variable corresponding to each camp-resource set combo
preds <- t(preds)
preds <- melt(preds)

#add the predicted values to the camp number, variable, and model attributes
other <- out[rep(seq_len(nrow(out)), each=length(xvals)),]

det.fits <- cbind(preds, other, count=rep(xvals, dim(out)[1]))

#relevel "variable" in tot dataframe to change order
levels(tot$variable)
tot$variable <- factor(tot$variable, levels = c("tot_meat", "tot_tuber", "tot_rat_only", "tot", "tot_rat"))

#plot the diminishing returns resources with deterministic functions
b <- ggplot(tot,aes(x=count,y=value,color=variable))+ geom_point(size=1.5) + facet_wrap(~Camp.number,scale=("free")) + ylab("Energy acquired (kcal)") + xlab("Day in camp") + theme_classic() + theme(legend.justification=c(1,0),legend.position=c(1,0)) + scale_color_discrete(labels=c("Meat", "Tubers", "Rattan","Wild food total", "Wild food total + rattan")) + guides(color=guide_legend(title=NULL))

#add the deterministic fits
b + geom_line(det.fits, mapping=aes(x=count, y=value))

######### PLOT FIGURE 3A #########
#relevel "variable" in final.df dataframe to change order
levels(final.df$variable)
final.df$variable <- factor(final.df$variable, levels = c("tot_meat", "tot_tuber", "tot_rat_only", "tot", "tot_rat"))

fig3A <- ggplot(final.df, aes(x=b,y=actual))+geom_point(size=4.5, aes(fill=variable), colour="black", pch=21)+theme_classic()+geom_abline(intercept=0,slope=1, linetype=2) +labs(x="Predicted residence time (days)", y="Observed residence time (days)", size=20)+geom_smooth(data=final.df, method = "lm", fullrange=F, colour="black")+scale_y_continuous(limits = c(0, 30))+ theme(legend.position=c(.8,.2), legend.text=element_text(size=20), axis.text=element_text(size=20), axis.title=element_text(size=20), axis.title.y=element_text(vjust=1), axis.title.x=element_text(vjust=-.4))+ scale_fill_discrete(labels=c("Meat", "Tubers", "Rattan","Wild food total", "Wild food total + rattan")) + guides(fill=guide_legend(title=NULL))
#+ annotate("text", x = 1, y = 30, label = "A", fontface="bold", size=10)

fig3A

# Stats for Fig. 3A
mod1 <- lm(actual~b, data=final.df)    #model run on data
confint(mod1)
summary(mod1)

#run models on individual resource sets and output 95% confidence intervals and r2
mods <- final.df %>% 
  group_by(variable) %>%
  do(model = lm(actual ~ b, data = .)) %>%
  dplyr:::mutate(slope=summary(model)$coeff[2],
                 rsquare=summary(model)$r.square,
                 l.ci=confint(model)[2],
                 u.ci=confint(model)[4])
mods


########### PLOT FIGURE 3B ############
fig3B <- ggplot(final.df, aes(x=b-actual, fill=variable)) + geom_histogram(binwidth=3, colour="black")+theme_classic()+labs(x="Predicted - actual residence time (days)", y="Frequency") + scale_fill_discrete(labels=c("Meat", "Tubers", "Rattan","Wild food total", "Wild food total + rattan")) + theme(legend.position=c(.8,.8), legend.text=element_text(size=20), axis.text=element_text(size=20), axis.title=element_text(size=20), axis.title.y=element_text(vjust=1), axis.title.x=element_text(vjust=-.4))+ guides(fill=guide_legend(title=NULL))
#+ annotate("text", x = -14, y = 6, label = "B", fontface="bold", size=10)

fig3B

# Stats for Fig. 3B (one way t-test of predicted minus actual residence times). Note that this is equivalent to doing a paired t-test of the two vectors.
t.test(final.df$b-final.df$actual)


# Random stats included in paper: difference between predicted and actual residence times by resource.
T2_MVT <- final.df %>%
  group_by(variable) %>% 
  summarize(avg.diff = mean(b-actual), SE = sd(b-actual)/sqrt(length(b)))

names(T2_MVT) <- c("Resource Set", "Mean_Difference", "SE")
T2_MVT <- T2_MVT[with(T2_MVT, order(-Mean_Difference)),]
T2_MVT[,1] <- c("Total wild food and rattan", "Rattan", "Total wild food", "Meat", "Tubers")


library(xtable)
print(xtable(T2_MVT, include.rownames=F, digits=3), floating=F)
