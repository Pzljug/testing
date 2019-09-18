<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Days of the Week</name>
   <tag></tag>
   <elementGuidId>0107b1ac-040f-436f-a97c-1c3fa0407ead</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>form-group  </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                                            
                            Days of the Week
                                                                                            *
            
                                                                                
                    
                                
    
        
                            Days of the Week
                                                            Monday
                                                                Tuesday
                                                                Wednesday
                                                                Thursday
                                                                Friday
                                                                Saturday
                                                                Sunday
                                    
        
            
                
            
        
        
            
                
            
        
    
    
        $(function(){
            $(&quot;#blob_dow&quot;).select2({theme: &quot;bootstrap&quot;, width:'100%'});
            $(&quot;#blob_dow-all&quot;).click(function(){
                var $sel = $(&quot;#blob_dow&quot;);
                var $sib = $sel.siblings('.select2');
                if($sib.hasClass('select2-container--disabled')){
                    return false;
                }
                $sel.find('option').each(function(){
                    if($(this).prop('disabled')) {
                        $(this).prop('selected', false);
                    } else {
                        $(this).prop('selected',true);
                    }
                });
                $sel.trigger('change');
            });
            $(&quot;#blob_dow-none&quot;).click(function(){
                var $sel = $(&quot;#blob_dow&quot;);
                var $sib = $sel.siblings('.select2');
                if($sib.hasClass('select2-container--disabled')){
                    return false;
                }
                $sel.find('option').each(function(){
                    $(this).prop('selected',false);
                });
                $sel.trigger('change');
            });
        });
    


                    
                                    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;form_edit&quot;)/div[@class=&quot;panel panel-sitespecific&quot;]/div[@class=&quot;panel-body&quot;]/div[@class=&quot;form-group&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form_edit']/div/div[3]/div[9]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='*'])[6]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='(ex 16:00)'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[9]</value>
   </webElementXpaths>
</WebElementEntity>
