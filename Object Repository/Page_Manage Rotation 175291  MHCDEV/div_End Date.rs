<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_End Date</name>
   <tag></tag>
   <elementGuidId>ee58b350-8763-46e1-8432-0cd25d5353fc</elementGuidId>
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
                                            
                            End Date
                                                                                            *
            
                                                                                
                    
                                
    
    
    

        $('#id_rotation_date_end').datepicker({
            todayBtn: &quot;linked&quot;,
            todayHighlight: true
        }).on('changeDate', function(e){
            $(this).trigger('input');
        });
    



                    
                                    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;form_edit&quot;)/div[@class=&quot;panel panel-sitespecific&quot;]/div[@class=&quot;panel-body&quot;]/div[@class=&quot;row&quot;]/div[@class=&quot;col-sm-6&quot;]/div[@class=&quot;form-group&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form_edit']/div/div[3]/div[7]/div[2]/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='*'])[3]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[7]/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
